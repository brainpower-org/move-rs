use model;
use rusoto_dynamodb::{
    DynamoDb, DynamoDbClient, PutItemError, PutItemInput, PutItemOutput, ScanError, ScanInput,
};

#[derive(Debug)]
pub struct Move<T> {
    db: T,
    table_name: String,
}

#[derive(FromForm)]
pub struct CreatePersonPayload {
    name: String,
}

#[derive(FromForm)]
pub struct CreateBuildingPayload {
    geo_coordinate: model::GeoCoordinate,
    name: String,
}

impl<T: DynamoDb> Move<T> {
    pub fn new() -> Move<DynamoDbClient> {
        let db = DynamoDbClient::new(rusoto_core::Region::EuCentral1);
        let table_name = String::from("rust-skillgroup");
        Move { db, table_name }
    }

    pub fn create_person(
        &self,
        person_payload: CreatePersonPayload,
    ) -> Result<PutItemOutput, PutItemError> {
        let person = model::Person::from_name(person_payload.name);

        let put_person = PutItemInput {
            item: serde_dynamodb::to_hashmap(&person).unwrap(),
            table_name: self.table_name.clone(),
            ..Default::default()
        };

        self.db.put_item(put_person).sync()
    }

    pub fn read_persons(&self) -> Result<Vec<model::Person>, ScanError> {
        let mut scan_input = ScanInput::default();
        scan_input.table_name = self.table_name.clone();

        match self.db.scan(scan_input).sync() {
            Ok(scan_output) => {
                println!("{:?}", scan_output.items);
                Ok(scan_output
                    .items
                    .unwrap_or_else(|| vec![])
                    .into_iter()
                    .map(|item| serde_dynamodb::from_hashmap::<model::Person>(item).unwrap())
                    .filter(|person| person.model_type == String::from("Person"))
                    .collect::<Vec<model::Person>>())
            }
            Err(scan_error) => Err(scan_error),
        }
    }

    pub fn create_building(
        &self,
        building_payload: CreateBuildingPayload,
    ) -> Result<PutItemOutput, PutItemError> {
        let CreateBuildingPayload {
            geo_coordinate,
            name,
        } = building_payload;
        let mut building = model::Building::from_geo_coordinate(geo_coordinate);
        building.name = name;

        let put_building = PutItemInput {
            item: serde_dynamodb::to_hashmap(&building).unwrap(),
            table_name: self.table_name.clone(),
            ..Default::default()
        };

        self.db.put_item(put_building).sync()
    }
}

#[cfg(test)]
mod test {
    use super::Move;
    use futures::prelude::*;
    use mocks::DynamoDbMock;
    use mocktopus::mocking::*;
    use model;
    use rusoto_core::RusotoFuture;
    use rusoto_dynamodb::*;
    use std::collections::HashMap;
    use std::time::{Duration, Instant};
    use tokio_timer::Delay;

    #[test]
    fn read_persons_failes() {
        let move_app = Move {
            db: DynamoDbMock {},
            table_name: String::from("test"),
        };

        DynamoDbMock::scan.mock_safe(|_, _| {
            MockResult::Return(
                Err(ScanError::Validation("This scan should fail".to_string())).into(),
            )
        });

        let persons = move_app.read_persons();
        assert!(persons.is_err());
    }

    #[test]
    fn read_persons_empty_result() {
        let move_app = Move {
            db: DynamoDbMock {},
            table_name: String::from("test"),
        };

        DynamoDbMock::scan.mock_safe(|_, _| {
            let deadline = Instant::now() + Duration::from_secs(3);
            let output = ScanOutput {
                ..Default::default()
            };
            let future = RusotoFuture::from_future(
                Delay::new(deadline)
                    .map_err(|_| ScanError::Validation("Invalid bucket".to_string()))
                    .map(|_| output),
            );

            MockResult::Return(future)
        });

        let persons = move_app.read_persons();
        assert!(persons.is_ok());
    }

    #[test]
    fn read_persons_filter_none_person_objects() {
        let move_app = Move {
            db: DynamoDbMock {},
            table_name: String::from("test"),
        };

        DynamoDbMock::scan.mock_safe(|_, _| {
            let deadline = Instant::now() + Duration::from_secs(3);

            let item_building = serde_dynamodb::to_hashmap(&model::Building {
                id: "87172779-07f0-456f-a046-b117550ce3e9".to_string(),
                model_type: "Building".to_string(),
                name: "HausH".to_string(),
                geo_coordinate: model::GeoCoordinate { lat: 0.0, lng: 0.0 },
                address: "".to_string(),
                phone_number: "".to_string(),
                email: "".to_string(),
            })
            .unwrap();

            let item_person = serde_dynamodb::to_hashmap(&model::Person {
                id: "87172779-07f0-456f-a046-b117550ce3e9".to_string(),
                model_type: "Person".to_string(),
                name: "HausH".to_string(),
            })
            .unwrap();

            let mut item_karpott = HashMap::new();
            item_karpott.insert(
                "id".to_string(),
                AttributeValue {
                    s: Some("18237".to_string()),
                    ..AttributeValue::default()
                },
            );

            let output = ScanOutput {
                count: Some(3),
                scanned_count: Some(3),
                items: Some(vec![item_person, item_building, item_karpott]),
                ..Default::default()
            };

            let future = RusotoFuture::from_future(
                Delay::new(deadline)
                    .map_err(|_| ScanError::Validation("Invalid bucket".to_string()))
                    .map(|_| output),
            );

            MockResult::Return(future)
        });

        let persons = move_app.read_persons();
        assert!(persons.is_ok());
        assert_eq!(persons.unwrap().len(), 1);
    }
}

use model;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemError, PutItemInput, PutItemOutput,
                      ScanError, ScanInput};

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
            Ok(scan_output) => Ok(
                scan_output
                    .items
                    .unwrap_or_else(|| vec![])
                    .into_iter()
                    .map(|item| {
                        serde_dynamodb::from_hashmap::<model::Person>(item).unwrap()
                    })
                    .filter(|person| person.model_type == String::from("Person"))
                    .collect::<Vec<model::Person>>(),
            ),
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
    use rusoto_core::RusotoFuture;
    use rusoto_dynamodb::*;
    use mocktopus::mocking::*;
    use std::time::{Duration, Instant};
    use futures::prelude::*;
    use tokio_timer::Delay;
    use mocks::DynamoDbMock;

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
}

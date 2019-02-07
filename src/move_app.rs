use futures::Future;
use model;
use rusoto_core::Region;
use rusoto_dynamodb::{
    AttributeDefinition, CreateTableInput, DynamoDb, DynamoDbClient, KeySchemaElement,
    ProvisionedThroughput, PutItemError, PutItemInput, PutItemOutput, ScanError, ScanInput,
};
use std::env;
use std::str::FromStr;

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
        let table_name = String::from("rust-skillgroup");
        let region = match env::var("AWS_DEFAULT_REGION").unwrap().as_ref() {
            "local" => Region::Custom {
                name: "local".to_owned(),
                endpoint: "http://dynamodb:8000".to_owned(),
            },
            region => Region::from_str(&region).expect("unknown aws region"),
        };
        let db = DynamoDbClient::new(region);

        // https://github.com/rusoto/rusoto/issues/1086
        match db
            .create_table(CreateTableInput {
                attribute_definitions: [AttributeDefinition {
                    attribute_name: "id".to_owned(),
                    attribute_type: "S".to_owned(),
                }]
                .to_vec(),
                global_secondary_indexes: Option::None,
                key_schema: [KeySchemaElement {
                    attribute_name: "id".to_owned(),
                    key_type: "HASH".to_owned(),
                }]
                .to_vec(),
                local_secondary_indexes: Option::None,
                provisioned_throughput: ProvisionedThroughput {
                    read_capacity_units: 100,
                    write_capacity_units: 100,
                },
                sse_specification: Option::None,
                stream_specification: Option::None,
                table_name: table_name.clone(),
            })
            .sync()
        {
            Ok(_) => println!("success"),
            Err(err) => println!("{:?}", err),
        };

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

    pub fn read_buildings(&self) -> Result<Vec<model::Building>, ScanError> {
        let mut scan_input = ScanInput::default();
        scan_input.table_name = self.table_name.clone();

        match self.db.scan(scan_input).sync() {
            Ok(scan_output) => {
                println!("{:?}", scan_output.items);
                Ok(scan_output
                    .items
                    .unwrap_or_else(|| vec![])
                    .into_iter()
                    .map(|item| serde_dynamodb::from_hashmap::<model::Building>(item).unwrap())
                    .filter(|person| person.model_type == String::from("Building"))
                    .collect::<Vec<model::Building>>())
            }
            Err(scan_error) => Err(scan_error),
        }
    }

    pub fn read_entries<M: model::DbModel>(&self) {
        let mut scan_input = ScanInput::default();
        scan_input.table_name = self.table_name.clone();

        match self.db.scan(scan_input).sync() {
            Ok(scan_output) => {
                println!("{:?}", scan_output.items);
                Ok(scan_output
                    .items
                    .unwrap_or_else(|| vec![])
                    .into_iter()
                    .map(|item| serde_dynamodb::from_hashmap::<M>(item).unwrap())
                    //.filter(|entry| entry.model_type == M::type_string().to_string())
                    .collect::<Vec<M>>())
            }
            Err(scan_error) => Err(scan_error),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use futures::future;
    use futures::prelude::*;
    use mocks::DynamoDbMock;
    use mocktopus::mocking::*;
    use model;
    use rusoto_core::RusotoFuture;
    use rusoto_dynamodb::*;
    use std::collections::HashMap;

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
    fn create_person_fails() {
        let move_app = Move {
            db: DynamoDbMock {},
            table_name: String::from("test"),
        };

        DynamoDbMock::put_item.mock_safe(|_, _| {
            MockResult::Return(
                Err(PutItemError::Validation(
                    "This put_item should fail".to_string(),
                ))
                .into(),
            )
        });

        let person = move_app.create_person(CreatePersonPayload {
            name: String::from("Testuser"),
        });
        assert!(person.is_err());
    }

    #[test]
    fn create_person_success() {
        let move_app = Move {
            db: DynamoDbMock {},
            table_name: String::from("test"),
        };

        DynamoDbMock::put_item.mock_safe(|_, input| {
            let output = PutItemOutput {
                ..Default::default()
            };

            MockResult::Return(RusotoFuture::from_future(future::ok(output)))
        });

        let person = move_app.create_person(CreatePersonPayload {
            name: String::from("Testuser"),
        });

        assert!(person.is_ok());
    }

    #[test]
    fn create_person_correct_input() {
        let move_app = Move {
            db: DynamoDbMock {},
            table_name: String::from("test"),
        };

        DynamoDbMock::put_item.mock_safe(|_, input| {
            assert_eq!("Bob", input.item.get("name").unwrap().clone().s.unwrap());
            MockResult::Return(Err(PutItemError::Validation("Irrelephant".to_string())).into())
        });

        move_app.create_person(CreatePersonPayload {
            name: String::from("Bob"),
        });
    }

    #[test]
    fn read_persons_empty_result() {
        let move_app = Move {
            db: DynamoDbMock {},
            table_name: String::from("test"),
        };

        DynamoDbMock::scan.mock_safe(|_, _| {
            let output = ScanOutput {
                ..Default::default()
            };
            MockResult::Return(RusotoFuture::from_future(future::ok(output)))
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

            MockResult::Return(RusotoFuture::from_future(future::ok(output)))
        });

        let persons = move_app.read_persons();
        assert!(persons.is_ok());
        assert_eq!(persons.unwrap().len(), 1);
    }
}

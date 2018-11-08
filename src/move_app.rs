use model;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, PutItemOutput, PutItemError, ScanError, ScanInput};

pub struct Move {
    db: DynamoDbClient,
}

#[derive(FromForm)]
pub struct SavePersonPayload {
    name: String
}

impl Move {
    pub fn new() -> Move {
        let db = DynamoDbClient::new(rusoto_core::Region::EuCentral1);
        Move { db }
    }

    pub fn save_person(&self, person_payload: SavePersonPayload) -> Result<PutItemOutput, PutItemError> {
        let person = model::Person::from_name(person_payload.name);

        let put_person = PutItemInput {
            item: serde_dynamodb::to_hashmap(&person).unwrap(),
            table_name: "rust-skillgroup".to_string(),
            ..Default::default()
        };

        self.db.put_item(put_person).sync()
    }

    pub fn get_persons(&self) -> Result<Vec<model::Person>, ScanError> {
        let mut scan_input = ScanInput::default();
        scan_input.table_name = String::from("rust-skillgroup");

        match self.db.scan(scan_input).sync() {
            Ok(scan_output) => Ok(
                scan_output
                    .items
                    .unwrap_or_else(|| vec![])
                    .into_iter()
                    .map(|item| serde_dynamodb::from_hashmap::<model::Person>(item).unwrap())
                    .collect::<Vec<model::Person>>(),
            ),
            Err(scan_error) => Err(scan_error),
        }
    }
}
use rocket::request::Form;
use rocket::response::status;
use rocket::State;
use rocket_contrib::Json;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, ScanInput};
use std::error::Error;

use model;

/**
* 
* Call with curl
* 
* curl -X POST \
 http://localhost:8000/person \
 -H 'Content-Type: application/x-www-form-urlencoded' \
 -d 'id=2&name=rust-update'
*/
#[post("/", data = "<person>")]
pub fn put_person(client: State<DynamoDbClient>, person: Form<model::Person>) -> String {
    let put_person = PutItemInput {
        item: serde_dynamodb::to_hashmap(&person.into_inner()).unwrap(),
        table_name: "rust-skillgroup".to_string(),
        ..Default::default()
    };

    match client.put_item(put_person).sync() {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}

#[get("/all")]
pub fn get_persons(
    client: State<DynamoDbClient>,
) -> Result<Json<Vec<model::Person>>, status::NotFound<String>> {
    let mut scan_input = ScanInput::default();
    scan_input.table_name = String::from("rust-skillgroup");

    match client.scan(scan_input).sync() {
        Ok(scan_output) => Ok(Json(
            scan_output
                .items
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(|item| serde_dynamodb::from_hashmap::<model::Person>(item).unwrap())
                .collect::<Vec<model::Person>>(),
        )),
        Err(scan_error) => Err(status::NotFound(scan_error.description().to_string())),
    }
}

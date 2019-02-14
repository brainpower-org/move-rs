use rocket::request::Form;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::Json;
use std::error::Error;

use model;
use move_app;

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
pub fn put_person(
    app: State<move_app::Move<rusoto_dynamodb::DynamoDbClient>>,
    person: Form<move_app::CreatePersonPayload>,
) -> String {
    match app.create_person(person.into_inner()) {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}

#[get("/all")]
pub fn get_persons(
    app: State<move_app::Move<rusoto_dynamodb::DynamoDbClient>>,
) -> Result<Json<Vec<model::Person>>, status::NotFound<String>> {
    app.read_entries::<model::Person>()
        .map(|persons| Json(persons))
        .map_err(|err| status::NotFound(err.description().to_string()))
}

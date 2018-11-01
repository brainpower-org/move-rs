use rocket::request::Form;
use rocket::response::status;
use rocket::State;
use rocket_contrib::Json;
use std::error::Error;

use move_app;
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
pub fn put_person(app: State<move_app::Move>, person: Form<move_app::SavePersonPayload>) -> String {
    match app.save_person(person.into_inner()) {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}

#[get("/all")]
pub fn get_persons(
    app: State<move_app::Move>,
) -> Result<Json<Vec<model::Person>>, status::NotFound<String>> {
    app.get_persons()
        .map(|persons| Json(persons))
        .map_err(|err| status::NotFound(err.description().to_string()))
}

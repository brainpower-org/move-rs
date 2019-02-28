use rocket::request::Form;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::Json;
use std::error::Error;

use model;
use move_app;

#[derive(FromForm, Debug)]
pub struct CreateFloorPayload {
    name: String,
    description: String,
    building_id: String,
    coordinates: Vec<(i32, i32)>,
    tags: Vec<String>,
}

#[post("/", data = "<floor_payload>")]
pub fn put_floor(
    app: State<move_app::Move<rusoto_dynamodb::DynamoDbClient>>,
    floor_payload: Form<CreateFloorPayload>,
) -> String {
    let CreateFloorPayload {
        name,
        description,
        building_id,
        coordinates,
        tags
    } = floor_payload.into_inner();

    let building = match app.read_entry::<model::Building>(&building_id) {
        Ok(b) => b,
        Err(e) => return format!("{:?}", e)
    };

    let floor = model::Floor {
      name,
        description,
        coordinates,
        tags,
        building,
        ..Default::default()
    };

    match app.create_entry(floor) {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}
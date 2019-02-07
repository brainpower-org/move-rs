use rocket::request::Form;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::Json;
use std::error::Error;

use model;
use move_app;
use rocket_contrib::json::Json;

#[post("/", data = "<building>")]
pub fn put_building(
    app: State<move_app::Move<rusoto_dynamodb::DynamoDbClient>>,
    building: Form<move_app::CreateBuildingPayload>,
) -> String {
    match app.create_building(building.into_inner()) {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}

#[get("/all")]
pub fn get_buildings(
    app: State<move_app::Move<rusoto_dynamodb::DynamoDbClient>>,
) -> Result<Json<Vec<model::Building>>, status::NotFound<String>> {
    app.read_buildings()
        .map(|buildings| Json(buildings))
        .map_err(|err| status::NotFound(err.description().to_string()))
}

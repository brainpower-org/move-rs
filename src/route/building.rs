use rocket::request::Form;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::Json;
use std::error::Error;

use model;
use move_app;


#[derive(FromForm, Debug)]
pub struct CreateBuildingPayload {
    geo_coordinate: model::GeoCoordinate,
    name: String,
}

#[post("/", data = "<building_payload>")]
pub fn put_building(
    app: State<move_app::Move<rusoto_dynamodb::DynamoDbClient>>,
    building_payload: Form<CreateBuildingPayload>,
) -> String {
    let CreateBuildingPayload {
        geo_coordinate,
        name,
    } = building_payload.into_inner();

    let building = model::Building {
        geo_coordinate,
        name,
        ..Default::default()
    };

    match app.create_entry(building) {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}

#[get("/all")]
pub fn get_buildings(
    app: State<move_app::Move<rusoto_dynamodb::DynamoDbClient>>,
) -> Result<Json<Vec<model::Building>>, status::NotFound<String>> {
    app.read_entries::<model::Building>()
        .map(|buildings| Json(buildings))
        .map_err(|err| status::NotFound(err.description().to_string()))
}

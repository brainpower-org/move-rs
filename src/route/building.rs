use rocket::request::Form;
use rocket::State;

use move_app;

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

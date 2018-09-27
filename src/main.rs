#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate futures;
extern crate rocket;
extern crate rocket_contrib;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_dynamodb;

use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::response::status;
use rocket::State;
use rocket_contrib::Json;
use rusoto_dynamodb::{
    DynamoDb, DynamoDbClient, PutItemInput, ScanInput,
};
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

mod model;
use model::*;

#[derive(Serialize, Deserialize)]
enum Place {

    /**
     * A piece of furniture positioned
     * on a floor plan, e.g. a desk or rack
     */
    Furniture {
        id: String,
        building: Building,
        floor: Floor,
        coordinates: Vec<(i32, i32)>,
        tags: Vec<String>,
        model_type: String,
    },
    /**
     * A piece of utility equipment,
     * e.g. a dish washer, washing machine, printer
     */
    Appliance {
        description: String,
        id: String,
        building: Building,
        floor: Floor,
        coordinates: Vec<(i32, i32)>,
        tags: Vec<String>,
        model_type: String,
    }
}

#[derive(Serialize, Deserialize)]
enum Region {
    /**
     * A generic area, e.g. room, stairwell
     */
    Area {
        id: String,
        name: String,
        building: Building,
        floor: Floor,
        coordinates: Vec<(i32, i32)>,
        label: String,
        tags: Vec<String>,
        model_type: String,
    },
    /**
     * A named meeting room
     */
    MeetingRoom {
        id: String,
        name: String,
        description: String,
        building: Building,
        floor: Floor,
        coordinates: Vec<(i32, i32)>,
        label: String,
        tags: Vec<String>,
        model_type: String,
    },
    /**
     * A loosely defined (project) work space
     */
    Workspace {
        id: String,
        name: String,
        description: String,
        building: Building,
        floor: Floor,
        coordinates: Vec<(i32, i32)>,
        label: String,
        tags: Vec<String>,
        model_type: String,
    },
    /**
     * Rooms housing shared functions such 
     * as rest rooms, showers, elevators
     */
    Facility {
        id: String,
        name: String,
        description: String,
        building: Building,
        floor: Floor,
        coordinates: Vec<(i32, i32)>,
        label: String,
        tags: Vec<String>,
        model_type: String,
    },
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/employees")]
fn get_employees(
    client: State<DynamoDbClient>,
) -> Result<Json<Vec<Employee>>, status::NotFound<String>> {
    let mut scan_input = ScanInput::default();
    scan_input.table_name = String::from("rust-skillgroup");

    match client.scan(&scan_input).sync() {
        Ok(scan_output) => Ok(Json(
            scan_output
                .items
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(|item| serde_dynamodb::from_hashmap::<Employee>(item).unwrap())
                .collect::<Vec<Employee>>(),
        )),
        Err(scan_error) => Err(status::NotFound(scan_error.description().to_string())),
    }
}

/**
 * Call with curl
 * 
 * curl -X POST \
  http://localhost:8000/employee \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'id=2&name=rust-update'
 */
#[post("/employee", data = "<employee>")]
fn put_employee(client: State<DynamoDbClient>, employee: Form<Employee>) -> String {
    let put_employee = PutItemInput {
        item: serde_dynamodb::to_hashmap(&employee.into_inner()).unwrap(),
        table_name: "rust-skillgroup".to_string(),
        ..Default::default()
    };

    match client.put_item(&put_employee).sync() {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}

#[get("/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let client = DynamoDbClient::simple(rusoto_core::Region::EuCentral1);

    rocket::ignite()
        .mount(
            "/",
            routes![
index,
get_employees,
put_employee,
files
],
        ).manage(client)
        .launch();
}

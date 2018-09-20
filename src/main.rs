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
extern crate uuid;

use rocket::request::Form;
use rocket::response::NamedFile;
use rocket::response::status;
use rocket::State;
use rocket_contrib::Json;
use rusoto_core::Region;
use rusoto_dynamodb::{
    DynamoDb, DynamoDbClient, PutItemInput, ScanInput,
};
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

#[derive(FromForm, Serialize, Deserialize)]
struct Employee {
    id: String,
    name: String,
    model_type: String,
}

#[derive(Serialize, Deserialize)]
enum Place {
    Desk {
        id: String,
        building: String,
        floor: String,
        coordinate: Vec<(i32, i32)>,
        image: String,

        employee: Option<Employee>,
    },
    Printer {
        id: String,
        building: String,
        floor: String,
        coordinate: Vec<(i32, i32)>,
        image: String,
    },
    Area {
        id: String,
        building: String,
        floor: String,
        coordinate: Vec<(i32, i32)>,

        label: String,
    }
}

impl Default for Employee {
    fn default() -> Employee {
        return Employee {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::from("NewUser"),
            model_type: String::from("Employee"),
        };
    }
}

impl Employee {
    fn from_name(name: String) -> Self {
        Employee {
            name,
            ..Employee::default()
        }
    }
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
    let client = DynamoDbClient::simple(Region::EuCentral1);

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

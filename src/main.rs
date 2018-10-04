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

use rocket::request::{Form, FromForm};
use rocket::response::status;
use rocket::response::NamedFile;
use rocket::State;
use rocket_contrib::Json;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemInput, ScanInput};
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
    },
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

#[get("/persons")]
fn get_persons(
    client: State<DynamoDbClient>,
) -> Result<Json<Vec<Person>>, status::NotFound<String>> {
    let mut scan_input = ScanInput::default();
    scan_input.table_name = String::from("rust-skillgroup");

    match client.scan(&scan_input).sync() {
        Ok(scan_output) => Ok(Json(
            scan_output
                .items
                .unwrap_or_else(|| vec![])
                .into_iter()
                .map(|item| serde_dynamodb::from_hashmap::<Person>(item).unwrap())
                .collect::<Vec<Person>>(),
        )),
        Err(scan_error) => Err(status::NotFound(scan_error.description().to_string())),
    }
}

/**
 * Call with curl
 * 
 * curl -X POST \
  http://localhost:8000/person \
  -H 'Content-Type: application/x-www-form-urlencoded' \
  -d 'id=2&name=rust-update'
 */
#[post("/person", data = "<person>")]
fn put_person(client: State<DynamoDbClient>, person: Form<Person>) -> String {
    let put_person = PutItemInput {
        item: serde_dynamodb::to_hashmap(&person.into_inner()).unwrap(),
        table_name: "rust-skillgroup".to_string(),
        ..Default::default()
    };

    match client.put_item(&put_person).sync() {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}

fn place(person: Option<Person>, mut seat: Seat) {
    seat.person = person
}

// http --verbose --form PUT localhost:8000/seat/1 id:=1
#[put("/seat/<seat_id>", data = "<person_id>")]
fn seat(seat_id: i32, person_id: Form<Id>) {
    println!("{} {:?}", seat_id, person_id);
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
struct Id {
    id: i32,
}

#[get("/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let client = DynamoDbClient::simple(rusoto_core::Region::EuCentral1);

    rocket::ignite()
        .mount("/", routes![index, get_persons, put_person, seat, files])
        .manage(client)
        .launch();
}

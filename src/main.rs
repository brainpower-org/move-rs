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

use rocket::request::{Form};
use rocket::response::NamedFile;
use rusoto_dynamodb::{DynamoDbClient};
use std::io;
use std::path::{Path, PathBuf};

mod model;
use model::*;

mod route;

#[derive(Serialize, Deserialize)]
enum Place {
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
        .mount("/", routes![index, seat, files])
        .mount("/person", routes![route::person::put_person, route::person::get_persons])
        .manage(client)
        .launch();
}

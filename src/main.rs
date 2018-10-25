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

use rocket::response::NamedFile;
use rusoto_dynamodb::DynamoDbClient;
use std::io;
use std::path::{Path, PathBuf};

mod model;
mod route;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let client = DynamoDbClient::simple(rusoto_core::Region::EuCentral1);

    rocket::ignite()
        .mount("/", routes![index, files])
        .mount(
            "/person",
            routes![route::person::put_person, route::person::get_persons],
        )
        .mount("/seat", routes![route::seat::get_seat])
        .manage(client)
        .launch();
}

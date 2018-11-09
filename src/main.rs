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

#[cfg(test)]
extern crate mocktopus;
#[cfg(test)]
extern crate tokio_timer;

use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};

mod model;
mod move_app;
mod route;

#[cfg(test)]
mod mocks;

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let app = move_app::Move::<rusoto_dynamodb::DynamoDbClient>::new();

    rocket::ignite()
        .mount("/", routes![index, files])
        .mount("/building", routes![route::building::put_building])
        .mount(
            "/person",
            routes![route::person::put_person, route::person::get_persons],
        )
        .mount("/seat", routes![route::seat::get_seat])
        .manage(app)
        .launch();
}

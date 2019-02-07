#![feature(plugin, proc_macro_hygiene, decl_macro)]
extern crate dotenv;
extern crate futures;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rusoto_core;
extern crate rusoto_dynamodb;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_dynamodb;
#[macro_use]
extern crate clap;

#[cfg(test)]
extern crate mocktopus;
#[cfg(test)]
extern crate tokio_timer;

use rocket::response::NamedFile;
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;

mod config;
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

fn start_app() {
    let app = move_app::Move::<rusoto_dynamodb::DynamoDbClient>::new();

    rocket::ignite()
        .mount("/", routes![index, files])
        .mount("/building", routes![route::building::put_building])
        .mount(
            "/person",
            routes![route::person::put_person, route::person::get_persons],
        )
        .mount("/seat", routes![route::seat::put_to_seat])
        .manage(app)
        .launch();
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (@arg preflight: -p "only perform preflight checks")
    )
    .get_matches();

    // let has_relevant_vars = ...;
    let env_config = config::MoveConfig::from_vars();
    let env_result = config::validate_config(env_config);

    if env_result.is_err() {
        println!("{:?}", env_result.err().unwrap());
        exit(1)
    }

    if matches.is_present("preflight") {
        exit(0)
    }
    start_app();
}

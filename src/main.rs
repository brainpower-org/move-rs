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

use dotenv::dotenv;
use rocket::response::NamedFile;
use std::env;
use std::io;
use std::path::{Path, PathBuf};
use std::process::exit;

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

fn validate_config() -> Result<String, String> {
    let config = MoveConfig::from_vars();

    match dotenv() {
        Ok(ref r) if config.is_empty() => Ok(format!("using env vars from: {:?}", r)),
        Ok(ref r) => Err(format!(
            "Mixing env vars and .env file is not supported. You have two options \n \
             1. Delete file: {:?} \n \
             2. Unset env vars {:?}",
            r,
            config.valid_keys()
        )),
        Err(dotenv::Error::Io(e)) => Err(format!("{}", e)),
        Err(dotenv::Error::LineParse(key)) => Err(format!("found invalid line in .env: {:?}", key)),
        Err(dotenv::Error::EnvVar(key)) => Err(format!("error: {:?}", key)),
    }
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
        .mount("/seat", routes![route::seat::get_seat])
        .manage(app)
        .launch();
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (@arg preflight: -p "only perform preflight checks")
    ).get_matches();

    // let has_relevant_vars = ...;
    let env_result =  validate_config();

    if env_result.is_err() {
        println!("{}", env_result.err().unwrap());
        exit(1)
    }

    if matches.is_present("preflight") {
        exit(0)
    }
    start_app();
}

#[derive(Debug)]
struct MoveConfig {
    id: ConfigItem,
    key: ConfigItem,
}

impl MoveConfig {
    fn from_vars() -> MoveConfig {
        let mut config = MoveConfig::new();
        config.id.value = std::env::var(&config.id.name);
        config.key.value = std::env::var(&config.key.name);

        config
    }

    pub fn is_empty(&self) -> bool {
        self.id.value.is_err() && self.key.value.is_err()
    }

    pub fn is_valid(&self) -> bool {
        self.id.value.is_ok() && self.key.value.is_ok()
    }

    pub fn valid_keys(&self) -> Vec<&String> {
        let valid_keys = vec![&self.id, &self.key];
        valid_keys
            .iter()
            .filter_map(|key| { 
                if key.value.is_ok() {
                    Some(&key.name)
                } else {
                    None
                }
            })
            .collect()
    }

    fn new() -> Self {
        MoveConfig {
            id: ConfigItem {
                name: "AWS_ACCESS_KEY_ID".to_string(),
                value: Err(std::env::VarError::NotPresent),
            },
            key: ConfigItem {
                name: "AWS_SECRET_ACCESS_KEY".to_string(),
                value: Err(std::env::VarError::NotPresent),
            },
        }
    }
}

#[derive(Debug)]
struct ConfigItem {
    name: String,
    value: Result<String, std::env::VarError>,
}

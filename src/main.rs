#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate futures;
extern crate rocket;
extern crate rusoto_core;
extern crate rusoto_credential;
extern crate rusoto_dynamodb;

use futures::future::Future;
use rocket::response::NamedFile;
use rusoto_core::request::HttpClient;
use rusoto_core::Region;
use rusoto_credential::{EnvironmentProvider, ProvideAwsCredentials};
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput, ScanInput};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[get("/area/<area..>")]
fn area(area: PathBuf) -> String {
    format!("Hello, {:?}!", area)
}

#[get("/resource/<name>")]
fn resource(name: String) -> String {
    // TODO ensure name has no invalid characters (cmd execution)
    let output = Command::new("find")
        .arg("data")
        .arg("-name")
        .arg(&name)
        .arg("-type")
        .arg("f")
        .arg("-not")
        .arg("-path")
        .arg("*/\\.*")
        .output()
        .expect("failed to execute process");
    String::from_utf8_lossy(&output.stdout).to_string()
}

#[put("/resource/<name>/<area..>")]
fn put_resource(name: String, area: PathBuf) -> String {
    // TODO ensure name has no invalid characters (cmd execution)
    format!("Test {:?}/{}!", area, name)
}

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("static/index.html")
}

#[get("/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let creds = EnvironmentProvider.credentials().wait().unwrap();
    //println!("{:?}", creds);
    let client = DynamoDbClient::simple(Region::UsEast2);

    let list_tables_input: ListTablesInput = Default::default();

    match client.list_tables(&list_tables_input).sync() {
        Ok(output) => match output.table_names {
            Some(table_name_list) => {
                println!("Tables in database:");

                for table_name in table_name_list {
                    println!("{}", table_name);
                    let mut scan_input = ScanInput::default();
                    scan_input.table_name = String::from("Testtable");

                    match client.scan(&scan_input).sync() {
                        Ok(scan_output) => {
                            println!("{:?}", scan_output);
                        }
                        Err(scan_error) => {
                            println!("{:?}", scan_error);
                        }
                    };
                }
            }
            None => println!("No tables in database!"),
        },
        Err(error) => {
            println!("Error: {:?}", error);
        }
    };

    rocket::ignite()
        .mount("/", routes![area, resource, put_resource, index, files])
        .launch();
}

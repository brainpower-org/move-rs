#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate futures;
extern crate rocket;
extern crate rusoto_core;
extern crate rusoto_dynamodb;

use rocket::request::{Form, FromForm};
use rocket::response::NamedFile;
use rocket::State;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, PutItemInput, ScanInput};
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(FromForm)]
struct Employee {
    id: String,
    name: String,
}

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

#[get("/tables")]
fn list_tables(client: State<DynamoDbClient>) -> String {
    let mut scan_input = ScanInput::default();
    scan_input.table_name = String::from("rust-skillgroup");

    match client.scan(scan_input).sync() {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
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
    let mut put_employee = PutItemInput::default();
    let mut employee_id = AttributeValue::default();
    let mut employee_name = AttributeValue::default();
    let employee_data = employee.into_inner();

    employee_id.s = Some(employee_data.id);
    employee_name.s = Some(employee_data.name);

    put_employee.table_name = String::from("rust-skillgroup");
    put_employee.item = HashMap::new();
    put_employee.item.insert(String::from("id"), employee_id);
    put_employee
        .item
        .insert(String::from("name"), employee_name);

    match client.put_item(put_employee).sync() {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}

#[get("/<file..>", rank = 1)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    let client = DynamoDbClient::new(Region::EuCentral1);

    rocket::ignite()
        .mount(
            "/",
            routes![
                area,
                resource,
                put_resource,
                index,
                list_tables,
                put_employee,
                files
            ],
        ).manage(client)
        .launch();
}

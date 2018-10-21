use rocket::request::Form;

// http --verbose --form PUT localhost:8000/seat/1 id:=1
#[put("/<seat_id>", data = "<person_id>")]
pub fn get_seat(seat_id: i32, person_id: Form<Id>) {
    println!("{} {:?}", seat_id, person_id);
}

#[derive(Serialize, Deserialize, Debug, FromForm)]
pub struct Id {
    id: i32,
}

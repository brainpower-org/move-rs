extern crate uuid;

#[derive(Serialize, Deserialize, FromForm)]
pub struct Person {
    id: String,
    name: String,
    model_type: String,
}

impl Default for Person {
    fn default() -> Person {
        return Person {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::from("NewUser"),
            model_type: String::from("Person"),
        };
    }
}

impl Person {
    #[allow(dead_code)]
    pub fn from_name(name: String) -> Self {
        Person {
            name,
            ..Person::default()
        }
    }
}

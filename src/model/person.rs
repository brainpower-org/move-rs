extern crate uuid;

use model::DbModel;

#[derive(Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub model_type: String,
}

impl DbModel for Person {
    fn type_string() -> &'static str {
        "Person"
    }

    fn model_type(&self) -> &String {
        &self.model_type
    }
}

impl Default for Person {
    fn default() -> Person {
        return Person {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::from("NewUser"),
            model_type: String::from(Person::type_string()),
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

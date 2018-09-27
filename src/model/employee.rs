extern crate uuid;

#[derive(Serialize, Deserialize, FromForm)]
pub struct Employee {
    id: String,
    name: String,
    model_type: String,
}

impl Default for Employee {
    fn default() -> Employee {
        return Employee {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::from("NewUser"),
            model_type: String::from("Employee"),
        };
    }
}

impl Employee {
    fn from_name(name: String) -> Self {
        Employee {
            name,
            ..Employee::default()
        }
    }
}
use model::Building;
use model::DbModel;
use model::Floor;
use model::Person;

/**
 * Work place users can book into,
 * e.g. a seat at a desk or a place on a couch
 */
pub struct Seat {
    id: String,
    building: Building,
    floor: Floor,
    coordinates: Vec<(i32, i32)>,
    pub person: Option<Person>,
    tags: Vec<String>,
    pub model_type: String,
}

impl DbModel for Seat {
    fn type_string() -> &'static str {
        "Seat"
    }
    fn model_type(&self) -> String {
        self.model_type
    }
}

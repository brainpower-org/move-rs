use Building;
use Employee;
use Floor;

/**
 * Work place users can book into,
 * e.g. a seat at a desk or a place on a couch
 */
pub struct Seat {
    id: String,
    building: Building,
    floor: Floor,
    coordinates: Vec<(i32, i32)>,
    user: Option<Employee>,
    tags: Vec<String>,
    model_type: String,
}
use model::Building;
use model::Floor;

/**
 * A piece of utility equipment,
 * e.g. a dish washer, washing machine, printer
 */
#[derive(Serialize, Deserialize)]
pub struct Appliance {
    description: String,
    id: String,
    building: Building,
    floor: Floor,
    coordinates: Vec<(i32, i32)>,
    tags: Vec<String>,
    model_type: String,
}

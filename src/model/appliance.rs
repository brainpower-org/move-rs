use model::Building;
use model::Floor;

/**
 * A piece of utility equipment,
 * e.g. a dish washer, washing machine, printer
 */
#[derive(Serialize, Deserialize)]
pub struct Appliance {
    pub description: String,
    pub id: String,
    pub building: Building,
    pub floor: Floor,
    pub coordinates: Vec<(i32, i32)>,
    pub tags: Vec<String>,
    pub model_type: String,
}

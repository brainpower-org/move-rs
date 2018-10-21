use model::Building;
use model::Floor;

/**
 * A generic area, e.g. room, stairwell
 */
#[derive(Serialize, Deserialize)]
pub struct Area {
    id: String,
    name: String,
    building: Building,
    floor: Floor,
    coordinates: Vec<(i32, i32)>,
    label: String,
    tags: Vec<String>,
    model_type: String,
}

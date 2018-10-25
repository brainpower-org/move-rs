use model::Building;
use model::Floor;

/**
 * Rooms housing shared functions such
 * as rest rooms, showers, elevators
 */
#[derive(Serialize, Deserialize)]
pub struct Facility {
    id: String,
    name: String,
    description: String,
    building: Building,
    floor: Floor,
    coordinates: Vec<(i32, i32)>,
    label: String,
    tags: Vec<String>,
    model_type: String,
}

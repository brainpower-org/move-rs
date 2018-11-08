use model::Building;
use model::Floor;

/**
 * Rooms housing shared functions such
 * as rest rooms, showers, elevators
 */
#[derive(Serialize, Deserialize)]
pub struct Facility {
    pub id: String,
    pub name: String,
    pub description: String,
    pub building: Building,
    pub floor: Floor,
    pub coordinates: Vec<(i32, i32)>,
    pub label: String,
    pub tags: Vec<String>,
    pub model_type: String,
}

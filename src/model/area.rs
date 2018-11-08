use model::Building;
use model::Floor;

/**
 * A generic area, e.g. room, stairwell
 */
#[derive(Serialize, Deserialize)]
pub struct Area {
    pub id: String,
    pub name: String,
    pub building: Building,
    pub floor: Floor,
    pub coordinates: Vec<(i32, i32)>,
    pub label: String,
    pub tags: Vec<String>,
    pub model_type: String,
}

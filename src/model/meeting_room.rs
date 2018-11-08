use model::Building;
use model::Floor;

/**
 * A named meeting room
 */
#[derive(Serialize, Deserialize)]
pub struct MeetingRoom {
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

use model::Building;
use model::Floor;

/**
 * A loosely defined (project) work space
 */
#[derive(Serialize, Deserialize)]
pub struct Workspace {
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

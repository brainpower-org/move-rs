use model::Building;
use model::Floor;

/**
 * A piece of furniture positioned
 * on a floor plan, e.g. a desk or rack
 */
#[derive(Serialize, Deserialize)]
pub struct Furniture {
    pub id: String,
    pub building: Building,
    pub floor: Floor,
    pub coordinates: Vec<(i32, i32)>,
    pub tags: Vec<String>,
    pub model_type: String,
}

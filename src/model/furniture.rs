use model::Building;
use model::Floor;

/**
 * A piece of furniture positioned
 * on a floor plan, e.g. a desk or rack
 */
#[derive(Serialize, Deserialize)]
pub struct Furniture {
    id: String,
    building: Building,
    floor: Floor,
    coordinates: Vec<(i32, i32)>,
    tags: Vec<String>,
    model_type: String,
}

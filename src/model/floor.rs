use Building;

/**
 * Floor inside a building
 */
#[derive(Serialize, Deserialize)]
pub struct Floor {
    id: String,
    name: String,
    description: String,
    building: Building,
    coordinates: Vec<(i32, i32)>,
    tags: Vec<String>,
    model_type: String,
}
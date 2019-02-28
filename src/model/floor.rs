use model::DbModel;
use model::Building;

/**
 * Floor inside a building
 */
#[derive(Serialize, Deserialize)]
pub struct Floor {
    pub id: String,
    pub name: String,
    pub description: String,
    pub building: Building,
    pub coordinates: Vec<(i32, i32)>,
    pub tags: Vec<String>,
    pub model_type: String,
}

impl DbModel for Floor {
    fn type_string() -> &'static str {
        "Floor"
    }
    fn model_type(&self) -> &String {
        &self.model_type
    }
    fn get_id(&self) -> &String { &self.id}
}
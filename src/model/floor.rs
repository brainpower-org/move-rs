use model::DbModel;
use model::Building;
use model::Coordinate;

/**
 * Floor inside a building
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Floor {
    pub id: String,
    pub name: String,
    pub description: String,
    pub building: Building,
    pub coordinates: Vec<Coordinate>,
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

impl Default for Floor {
    fn default() -> Floor {
        return Floor {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::from("New Floor"),
            model_type: Floor::type_string().to_string(),
            description: Default::default(),
            building: Default::default(),
            coordinates: vec![],
            tags: vec![],
        };
    }
}
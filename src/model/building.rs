/**
 * A building housing floors
 */
#[derive(Serialize, Deserialize)]
pub struct Building {
    id: String,
    name: String,
    address: String,
    phone_number: String,
    email: String,
    geo_coordinates: (f32, f32),
    model_type: String,
}
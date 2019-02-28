use model::DbModel;
use rocket::http::RawStr;
use rocket::request::FromFormValue;

/**
 * A building housing floors
 */
#[derive(Serialize, Deserialize, Debug)]
pub struct Building {
    pub id: String,
    pub name: String,
    pub address: String,
    pub phone_number: String,
    pub email: String,
    pub geo_coordinate: GeoCoordinate,
    pub model_type: String,
}

impl DbModel for Building {
    fn type_string() -> &'static str {
        "Building"
    }
    fn model_type(&self) -> &String {
        &self.model_type
    }
    fn get_id(&self) -> &String { &self.id}
}

#[derive(FromForm, Serialize, Deserialize, Debug, PartialEq)]
pub struct GeoCoordinate {
    pub lat: f32,
    pub lng: f32,
}

impl<'v> FromFormValue<'v> for GeoCoordinate {
    type Error = &'v RawStr;

    fn from_form_value(form_value: &'v RawStr) -> Result<GeoCoordinate, &'v RawStr> {
        let pair: Vec<Result<f32, std::num::ParseFloatError>> = form_value
            .split(',')
            .map(|item| item.parse::<f32>())
            .collect();

        match pair.as_slice() {
            [Ok(lat), Ok(lng)] => Ok(GeoCoordinate {
                lat: *lat,
                lng: *lng,
            }),
            _ => Err(form_value),
        }
    }
}

impl Default for Building {
    fn default() -> Building {
        return Building {
            id: uuid::Uuid::new_v4().to_string(),
            name: String::from("New Building"),
            model_type: Building::type_string().to_string(),
            address: Default::default(),
            phone_number: Default::default(),
            email: Default::default(),
            geo_coordinate: GeoCoordinate {lat: 0.0, lng: 0.0},
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn geo_coordinate_from_form_value_valid_parsing() {
        let raw_str = RawStr::from_str("10.123,20");
        let result = GeoCoordinate::from_form_value(raw_str);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            GeoCoordinate {
                lat: 10.123,
                lng: 20.0,
            }
        );
    }

    #[test]
    fn geo_coordinate_from_form_value_to_many_params() {
        let raw_str = RawStr::from_str("10.123,20,30");
        let result = GeoCoordinate::from_form_value(raw_str);
        assert!(result.is_err());
    }

    #[test]
    fn geo_coordinate_from_form_value_cant_parse_to_float() {
        let raw_str = RawStr::from_str("10.123,jsdhgf,30");
        let result = GeoCoordinate::from_form_value(raw_str);
        assert!(result.is_err());
    }

    #[test]
    fn geo_coordinate_from_form_value_less_then_two_parts() {
        let raw_str = RawStr::from_str("10.123");
        let result = GeoCoordinate::from_form_value(raw_str);
        assert!(result.is_err());
    }
}

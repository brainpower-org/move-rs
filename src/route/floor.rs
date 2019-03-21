use rocket::request::Form;
use rocket::response::status;
use rocket::State;
use rocket_contrib::json::Json;
use std::error::Error;

use model;
use move_app;

#[derive(FromForm, Debug)]
pub struct CreateFloorPayload {
    name: String,
    description: String,
    building_id: String,
    coordinates: String,
    tags: String,
}

#[post("/", data = "<floor_payload>")]
pub fn put_floor(
    app: State<move_app::Move<rusoto_dynamodb::DynamoDbClient>>,
    floor_payload: Form<CreateFloorPayload>,
) -> String {
    let CreateFloorPayload {
        name,
        description,
        building_id,
        coordinates,
        tags
    } = floor_payload.into_inner();

    let building = match app.read_entry::<model::Building>(&building_id) {
        Ok(b) => b,
        Err(e) => return format!("{:?}", e)
    };

    let coordinate_list = match split_coordinates(coordinates) {
        Ok(c) => c,
        Err(e) => return e
    };

    let floor = model::Floor {
      name,
        description,
        coordinates: coordinate_list,
        tags: tags.split(',').map(|tag| tag.to_string()).collect(),
        building,
        ..Default::default()
    };

    match app.create_entry(dbg!(floor)) {
        Ok(scan_output) => format!("{:?}", scan_output),
        Err(scan_error) => format!("{:?}", scan_error),
    }
}

fn split_coordinates(coordinate_list: String) -> Result<Vec<(i32, i32)>, String> {

    let coordinate_pairs: Vec<Result<Vec<i32>, std::num::ParseIntError>> = coordinate_list
        .split('|')
        .map(|pair| {
            pair
                .split(',')
                .map(|item| item.parse::<i32>())
                .collect::<Result<Vec<i32>, std::num::ParseIntError>>()
        })
        .collect();

    coordinate_pairs.into_iter().map(|pair| {
        match pair {
            Ok(ref p) if p.len() == 2 => Ok((p[0], p[1])),
            Ok(p) =>  Err(format!("To many numbers in the coordinate: {:?}", p)),
            Err(e) => Err(format!("Could not parse corordinate as number: {:?}", e)),
        }
    }).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn split_coordinates_splits_correctly() {
        let coordinates = "0,1|2,3|4,5".to_string();
        let result = split_coordinates(coordinates);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            vec![(0,1), (2,3), (4,5)]
        );
    }

    #[test]
    fn split_coordinates_number_parse_error_leads_to_error() {
        let coordinates = "0,1|2,3|b,5".to_string();
        let result = split_coordinates(coordinates);
        assert!(result.is_err());
    }

    #[test]
    fn split_coordinates_to_many_numbers_in_pair_leads_to_error() {
        let coordinates = "0,1|2,3|4,5,6".to_string();
        let result = split_coordinates(coordinates);
        assert!(result.is_err());
    }

    fn split_coordinates_string_without_pipe_returns_one_coordinate() {
        let coordinates = "0,1".to_string();
        let result = split_coordinates(coordinates);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap(),
            vec![(0,1)]
        );
    }
}
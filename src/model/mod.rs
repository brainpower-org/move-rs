mod appliance;
mod area;
mod building;
mod facility;
mod floor;
mod furniture;
mod meeting_room;
mod person;
mod seat;
mod workspace;
mod coordinate;

pub use self::appliance::Appliance;
pub use self::area::Area;
pub use self::building::{Building, GeoCoordinate};
pub use self::coordinate::Coordinate;
pub use self::facility::Facility;
pub use self::floor::Floor;
pub use self::furniture::Furniture;
pub use self::meeting_room::MeetingRoom;
pub use self::person::Person;
pub use self::seat::Seat;
pub use self::workspace::Workspace;

pub trait DbModel {
    fn type_string() -> &'static str;
    fn model_type(&self) -> &String;
    fn get_id(&self) -> &String;
}

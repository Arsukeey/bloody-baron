use crate::character::{NUMBER_OF_CHARS};
use crate::map::{RoomType};

#[derive(Copy, Clone)]
pub struct Protag {
    pub trust_table: [bool; NUMBER_OF_CHARS],
    pub location: RoomType
}

impl Protag {
    pub fn new() -> Self {
        let trust_table = [false; NUMBER_OF_CHARS];
        let location = RoomType::MainHall;
        Self {
            trust_table,
            location
        }
    }
}
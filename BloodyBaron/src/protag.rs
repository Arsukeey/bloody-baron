use crate::character::{NUMBER_OF_CHARS};
use crate::map::{RoomType};

#[derive(Copy, Clone)]
pub struct Protag {
    pub trust_table: [bool; NUMBER_OF_CHARS],
    pub location: RoomType,
    pub moved: bool,
    pub investigation: bool,
    pub hindsight: bool,
    pub perfect_bluff: bool,
    pub night_owl: bool
}

impl Protag {
    pub fn new() -> Self {
        let trust_table = [false; NUMBER_OF_CHARS];
        let location = RoomType::MainHall;
        Self {
            trust_table,
            location,
            moved: false,
            investigation: false,
            hindsight: false,
            perfect_bluff: false,
            night_owl: false
        }
    }
}
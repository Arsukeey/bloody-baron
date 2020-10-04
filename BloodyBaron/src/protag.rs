use crate::character::{NUMBER_OF_CHARS};
use crate::map::{RoomType};

#[derive(Copy, Clone)]
pub struct Protag {
    pub trust_table: [bool; NUMBER_OF_CHARS],
    pub location: RoomType,
    pub moved: bool,
    pub investigation: bool,
    pub celestial_call: bool,
    pub overhearing: bool,
    pub pick_lock: bool,
    pub hindsight: bool,
    pub perfect_bluff: bool,
    pub night_owl: bool,
    pub pitiful_begging: bool,
    pub cold_execution: bool,
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
            celestial_call: false,
            overhearing: false,
            pick_lock: false,
            hindsight: false,
            perfect_bluff: false,
            night_owl: false,
            pitiful_begging: false,
            cold_execution: false
        }
    }
}
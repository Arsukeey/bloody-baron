use crate::map::Map;
use crate::character::Character;

pub struct MovementPack<'a, 'b> {
    pub map: &'a mut Map,
    pub character: &'b mut Character,
    pub move_index: u32
}

impl<'a, 'b> MovementPack<'a, 'b> {
    pub fn execute() {
        
    }
}
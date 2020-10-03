use crate::map::Map;
use crate::character::Character;

pub struct MovementPack<'a, 'b> {
    pub map: &'a mut Map,
    pub character: &'b mut Character,
    pub move_origin: usize,
    pub move_index: usize
}

impl<'a, 'b> MovementPack<'a, 'b> {
    pub fn new(map: &'a mut Map, character: &'b mut Character,
    move_origin: usize, move_index: usize) -> Self {
        Self {
            map,
            character,
            move_origin,
            move_index,
        }
    }

    pub fn execute(&mut self) {
        let index = self.map.chars_in_rooms[self.move_origin]
            .iter().position(|x| *x == self.character.name).unwrap();
        self.map.chars_in_rooms[self.move_origin].remove(index);
        self.map.chars_in_rooms[self.move_index].push(self.character.name.clone());
    }
}
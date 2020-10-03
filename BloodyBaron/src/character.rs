use crate::map::Map;

pub struct Character {
    pub name: String,
    pub is_alive: bool,
    pub is_killer: bool,
    pub ability: fn(Vec<Character>, &Map) -> (),
    pub dialogue: fn(Vec<Character>, &Map) -> ()
}

impl Character {
    pub fn freya() {
        let name = "Freya";
        let is_alive = true;
    }

    pub fn ravi() {
        let name = "Ravi";
        let is_alive = true;
    }

    pub fn luna() {
        let name = "Luna";
        let is_alive = true;
    }

    pub fn john() {
        let name = "John";
        let is_alive = true;
    }

    pub fn amanda() {
        let name = "Amanda";
        let is_alive = true;
    }

    pub fn vincent() {
        let name = "Vincent";
        let is_alive = true;
    }

    pub fn larissa() {
        let name = "Larissa";
        let is_alive = true;
    }

    pub fn nobunaga() {
        let name = "Nobunaga";
        let is_alive = true;
    }

    pub fn odette() {
        let name = "Odette";
        let is_alive = true;
    }
}
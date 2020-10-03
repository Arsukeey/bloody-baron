use crate::{character::Character, map::Map};

pub fn investigation(chars: Vec<Character>, map: Map) {
    // allows to check where a character has been in the last 3 hours
}

pub fn celestial_call(chars: Vec<Character>, map: Map) {
    // protects one person through the day, not yourself
}

pub fn overhearing(chars: Vec<Character>, map: Map) {
    // identifies who's in nearby rooms
}

pub fn turn_of_the_cards(chars: Vec<Character>, map: Map) {
    // rises luck, by confusing the AI
}

pub fn pick_lock(chars: Vec<Character>, map: Map) {
    // allows you to pick locks of rooms and go inside
}

pub fn pitiful_begging(chars: Vec<Character>, map: Map) {
    // beg for your life (works only during the day)
}

pub fn exceptional_diplomacy(chars: Vec<Character>, map: Map) {
    // every dialogue path leads to gaining trust
}

pub fn cold_execution(chars: Vec<Character>, map: Map) {
    // allows you to kill someone (if there's someone nearby, you can get caught. if you kill the killer and get away, the game is over)
}

pub fn night_owl(chars: Vec<Character>, map: Map) {
    // meeting killer at night is not instant death, works as day
}
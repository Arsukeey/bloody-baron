use crate::map::Map;
use crate::abilities::*;
use crate::ai::AI;

pub const NUMBER_OF_CHARS: usize = 9;

#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub is_alive: bool,
    pub is_killer: bool,
    pub ability: fn(Vec<Character>, &Map) -> (),
    pub ai: AI,
    // pub dialogue: fn(&Character, Vec<Character>, &Map) -> ()
}

impl Character {
    pub fn freya(map: &Map) -> Self {
        let name = "Freya".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = investigation;
        let ai = AI::new(map);

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai
        }
    }

    pub fn ravi(map: &Map) -> Self {
        let name = "Ravi".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = celestial_call;
        let ai = AI::new(map);
        
        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai
        }
    }

    pub fn luna(map: &Map) -> Self {
        let name = "Luna".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = overhearing;
        let ai = AI::new(map);

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai
        }
    }

    pub fn john(map: &Map) -> Self {
        let name = "John".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = perfect_bluff;
        let ai = AI::new(map);

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai
        }
    }

    pub fn amanda(map: &Map) -> Self {
        let name = "Amanda".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = pick_lock;
        let ai = AI::new(map);

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai
        }
    }

    pub fn vincent(map: &Map) -> Self {
        let name = "Vincent".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = pitiful_begging;
        let ai = AI::new(map);

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai
        }
    }

    pub fn larissa(map: &Map) -> Self {
        let name = "Larissa".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = exceptional_diplomacy;
        let ai = AI::new(map);

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai
        }
    }

    pub fn chio(map: &Map) -> Self {
        let name = "Chio".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = cold_execution;
        let ai = AI::new(map);

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai
        }
    }

    pub fn odette(map: &Map) -> Self {
        let name = "Odette".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = night_owl;
        let ai = AI::new(map);

        Self {
            name,
            is_alive,
            is_killer,
            ability,
            ai
        }
    }
}
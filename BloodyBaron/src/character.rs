use crate::map::Map;
use crate::abilities::*;

#[derive(Clone)]
pub struct Character {
    pub name: String,
    pub is_alive: bool,
    pub is_killer: bool,
    pub ability: fn(Vec<Character>, &Map) -> (),
    // pub dialogue: fn(&Character, Vec<Character>, &Map) -> ()
}

impl Character {
    pub fn freya() -> Self {
        let name = "Freya".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = investigation;

        Self {
            name,
            is_alive,
            is_killer,
            ability
        }
    }

    pub fn ravi() -> Self {
        let name = "Ravi".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = celestial_call;
        
        Self {
            name,
            is_alive,
            is_killer,
            ability 
        }
    }

    pub fn luna() -> Self {
        let name = "Luna".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = overhearing;

        Self {
            name,
            is_alive,
            is_killer,
            ability
        }
    }

    pub fn john() -> Self {
        let name = "John".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = perfect_bluff;

        Self {
            name,
            is_alive,
            is_killer,
            ability
        }
    }

    pub fn amanda() -> Self {
        let name = "Amanda".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = pick_lock;

        Self {
            name,
            is_alive,
            is_killer,
            ability
        }
    }

    pub fn vincent() -> Self {
        let name = "Vincent".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = pitiful_begging;

        Self {
            name,
            is_alive,
            is_killer,
            ability
        }
    }

    pub fn larissa() -> Self {
        let name = "Larissa".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = exceptional_diplomacy;

        Self {
            name,
            is_alive,
            is_killer,
            ability
        }
    }

    pub fn chio() -> Self {
        let name = "Chio".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = cold_execution;

        Self {
            name,
            is_alive,
            is_killer,
            ability
        }
    }

    pub fn odette() -> Self {
        let name = "Odette".to_string();
        let is_alive = true;
        let is_killer = false;
        let ability = night_owl;

        Self {
            name,
            is_alive,
            is_killer,
            ability
        }
    }
}
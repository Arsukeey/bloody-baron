use crate::events::GameData;

pub fn investigation(game_data: &mut GameData) {
    game_data.protag.investigation = true;
}

pub fn celestial_call(game_data: &mut GameData) {
    game_data.protag.celestial_call = true;
}

pub fn overhearing(game_data: &mut GameData) {
    game_data.protag.overhearing = true;
}

pub fn perfect_bluff(game_data: &mut GameData) {
    game_data.protag.perfect_bluff = true
}

pub fn pick_lock(game_data: &mut GameData) {
    game_data.protag.pick_lock = true
}

pub fn pitiful_begging(game_data: &mut GameData) {
    game_data.protag.pitiful_begging = true
}

pub fn hindsight(game_data: &mut GameData) {
    game_data.protag.hindsight = true
}

pub fn cold_execution(game_data: &mut GameData) {
    game_data.protag.cold_execution = true
}

pub fn night_owl(game_data: &mut GameData) {
    game_data.protag.night_owl = true
}
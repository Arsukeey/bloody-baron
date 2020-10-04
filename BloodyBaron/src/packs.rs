use crate::map::{Map, RoomType, RoomTable, NUMBER_OF_ROOMS};
use crate::character::{Character, NUMBER_OF_CHARS};
use crate::protag::Protag;

#[derive(Clone)]
pub enum IdleChoices {
    MoveRoom,
    TalkToCharacter,
    ExamineCharacter,
    GoToRoom
}

#[derive(Clone)]
pub struct IdlePack;

pub struct MovementPack {
    pub protag_moves: bool,
    pub character_index: usize,
    pub move_origin: RoomType,
    pub move_index: RoomType
}

pub struct TrustPack {
    pub character_index: usize,
}

pub struct VotingPack {
    pub choice_name: String
}

pub struct ExecutionPack {
    pub choice_name: String
}

pub struct AbilityPack {
    pub name: String,
    pub chars: Vec<Character>
}
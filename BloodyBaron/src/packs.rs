use crate::map::{Map, RoomType, RoomTable, NUMBER_OF_ROOMS};
use crate::character::{Character, NUMBER_OF_CHARS};
use crate::protag::Protag;

#[derive(Clone)]
pub enum IdleChoices {
    MoveRoom,
    TalkToCharacter,
    ExamineCharacter
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

pub struct AbilityPack<'a, 'b> {
    pub name: String,
    pub map: &'a mut Map,
    pub character: &'b Character,
    pub chars: Vec<Character>
}

impl<'a, 'b> AbilityPack<'a, 'b> {
    pub fn execute(&mut self) {
        (self.character.ability)(self.chars.clone(), self.map);
    }
}

pub struct MurderPack<'a, 'b> {
    pub map: &'a mut Map,
    pub murdered: &'b mut Character,
    pub location: usize
}

impl<'a, 'b> MurderPack<'a, 'b> {
    pub fn execute(&mut self) {
        self.murdered.is_alive = false;
        self.map.has_corpse[self.location] = true
    }
}

pub struct CorpseDiscoveryPack<'a, 'c> {
    pub map: &'a mut Map,
    pub protag: &'c Protag
}

impl<'a, 'c> CorpseDiscoveryPack<'a, 'c> {
    pub fn execute(&mut self) {
        self.map.has_corpse[self.protag.location as usize] = false;
    }
}

pub struct TrialStartPack {
    pub line: String
}

pub struct TrialVotingPack {
    pub line: String
}

pub struct TrialExecutionPack<'b> {
    pub executed_char: &'b mut Character,
    pub line: String
}

impl<'b> TrialExecutionPack<'b> {
    pub fn execute(&mut self) {
        self.executed_char.is_alive = false;
    }
}

pub struct VictoryPack;

pub struct GameOverPack;
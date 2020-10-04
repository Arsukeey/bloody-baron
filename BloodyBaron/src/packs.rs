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
pub struct IdlePack {
    pub choices: Vec<String>,
    pub events: Vec<IdleChoices>,
    pub chars_indices: Vec<usize>,
    pub room_indices: Vec<usize>,
}

impl IdlePack {
    pub fn starter(map: &Box<Map>, characters: &Vec<Character>) -> Self {
        let mut choices = vec![];
        let mut events = vec![];
        let mut chars_indices = vec![];
        let mut room_indices = vec![];

        // initialize first room here
        for i in 0..NUMBER_OF_CHARS {
            chars_indices.push(i);
            room_indices.push(i);
            chars_indices.push(i);
            room_indices.push(i);
            choices.push(format!("{}{}", "Spend some time with ", characters[i].name));
            events.push(IdleChoices::TalkToCharacter);
            choices.push(format!("{}{}{}", "Examine ", characters[i].name, "'s profile"));
            events.push(IdleChoices::ExamineCharacter);
        }
        for i in 0..NUMBER_OF_ROOMS {
            if map.adjacency[0][i] == 1 {
                chars_indices.push(i);
                room_indices.push(i);
                events.push(IdleChoices::MoveRoom);
                choices.push(format!("Go to the {}", RoomTable[i]));
            }
        }

        Self {
            choices,
            events,
            chars_indices,
            room_indices
        }
    }
}

pub struct MovementPack {
    pub protag_moves: bool,
    pub character_index: usize,
    pub move_origin: RoomType,
    pub move_index: RoomType
}

pub struct TrustPack<'c> {
    pub protag: &'c mut Protag,
    pub character_index: usize,
    pub success: bool
}

impl<'c> TrustPack<'c> {
    pub fn execute(&mut self) -> bool {
        if self.success {
            self.protag.trust_table[self.character_index] = true
        }

        self.success
    }
}

pub struct DialoguePack<'b> {
    pub character: &'b mut Character,
    pub line: String,
    pub choices: Vec<String>,
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
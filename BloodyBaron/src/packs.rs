use crate::map::Map;
use crate::character::Character;
use crate::protag::Protag;

pub struct MovementPack<'a, 'b> {
    pub map: &'a mut Map,
    pub character: &'b mut Character,
    pub move_origin: usize,
    pub move_index: usize
}

impl<'a, 'b> MovementPack<'a, 'b> {
    pub fn execute(&mut self) {
        let index = self.map.chars_in_rooms[self.move_origin]
            .iter().position(|x| *x == self.character.name).unwrap();
        self.map.chars_in_rooms[self.move_origin].remove(index);
        self.map.chars_in_rooms[self.move_index].push(self.character.name.clone());
    }
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
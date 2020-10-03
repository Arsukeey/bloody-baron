use std::io::{stdin, stdout, Read, Write};
use std::collections::VecDeque;
use crate::character::Character;
use crate::map::Map;
use crate::packs::{
    MovementPack,
    TrustPack,
    DialoguePack,
    AbilityPack,
    MurderPack,
    CorpseDiscoveryPack,
    TrialStartPack,
    TrialVotingPack,
    TrialExecutionPack,
    VictoryPack,
    GameOverPack
};
use crate::protag::Protag;

pub enum EventType {
    Dialogue,
    TrustGain,
    TrustFail,
    Movement,
    Ability,
    Murder,
    TrialStart,
    TrialVoting,
    TrialExecution,
    Victory,
    GameOver
}

pub struct Event<'a, 'b, 'c> {
    pub event_type: EventType,
    pub movement_pack: Option<MovementPack<'a, 'b>>,
    pub trust_pack: Option<TrustPack<'c>>,
    pub dialogue_pack: Option<DialoguePack<'b>>,
    pub ability_pack: Option<AbilityPack<'a, 'b>>,
    pub murder_pack: Option<MurderPack<'a, 'b>>,
    pub corpse_discovery_pack: Option<CorpseDiscoveryPack<'a, 'c>>,
    pub trial_start_pack: Option<TrialStartPack>,
    pub trial_voting_pack: Option<TrialVotingPack>,
    pub trial_execution_pack: Option<TrialExecutionPack<'b>>,
    pub victory_pack: Option<VictoryPack>,
    pub game_over_pack: Option<GameOverPack>
}

pub struct EventQueue<'a, 'b, 'c> {
    pub events: VecDeque<Event<'a, 'b, 'c>>,
    pub characters: Vec<Character>,
    pub map: Map,
    pub protag: Protag
}

impl<'a, 'b, 'c>  EventQueue<'a, 'b, 'c>  {
    pub fn new() -> Self {
        let events = VecDeque::new();
        let map = Map::new();
        let characters = vec![
            Character::freya(&map),
            Character::ravi(&map),
            Character::luna(&map),
            Character::john(&map),
            Character::amanda(&map),
            Character::vincent(&map),
            Character::larissa(&map),
            Character::chio(&map),
            Character::odette(&map), 
        ];
        let protag = Protag::new();

        Self {
            events,
            characters,
            map,
            protag
        }
    }

    fn enqueue_event (&mut self, event: Event<'a, 'b, 'c>) {
        // called in case another event calls for a new event
        self.events.push_back(event);
    }

    pub fn execute_event(&mut self) {
        // here we execute all char actions
        println!("teste");
    }

    pub fn allow_next_event(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        let mut stdin = stdin();

        write!(stdout, "Press any key to continue...")?;
        stdout.flush()?;

            // Read a single byte and discard
        let mut buffer = [0u8];
        let _ = stdin.read(&mut buffer).unwrap();

        print!("{}[2J", 27 as char);
        Ok(())
    }
    
    pub fn poll_next_event(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}
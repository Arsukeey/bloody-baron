use std::io::{stdin, stdout, Read, Write};
use std::collections::VecDeque;
use crate::character::Character;
use crate::map::Map;
use crate::movement::MovementPack;

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

pub struct Event<'a, 'b> {
    pub event_type: EventType,
    pub movement_pack: Option<MovementPack<'a, 'b>>,

}

pub struct EventQueue<'a, 'b> {
    pub events: VecDeque<Event<'a, 'b>>,
    pub characters: Vec<Character>,
    pub map: Map
}

impl<'a, 'b>  EventQueue<'a, 'b>  {
    pub fn new() -> Self {
        let events = VecDeque::new();
        let characters = vec![
            Character::freya(),
            Character::ravi(),
            Character::luna(),
            Character::john(),
            Character::amanda(),
            Character::vincent(),
            Character::larissa(),
            Character::chio(),
            Character::odette(), 
        ];
        let map = Map::new();
        Self {
            events,
            characters,
            map
        }
    }

    fn enqueue_event (&mut self, event: Event<'a, 'b>) {
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
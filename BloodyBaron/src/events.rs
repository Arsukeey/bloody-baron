use std::io::{stdin, stdout, Read, Write};
use crate::character::Character;
use crate::map::Map;

pub struct Event {

}

pub struct EventQueue {
    pub events: Vec<Event>,
    pub characters: Vec<Character>,
    pub map: Map
}

impl EventQueue {
    pub fn new() -> Self {
        let events = vec![];
        let characters = vec![];
        let map = Map::new();
        Self {
            events,
            characters,
            map
        }
    }

    fn enqueue_event(&mut self, event: Event) {
        // called in case another event calls for a new event
        self.events.push(event);
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
    
    pub fn poll_next_event(&self) -> Option<Event> {
        return Some(Event{});
    }
}
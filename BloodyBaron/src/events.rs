use std::io::{stdin, stdout, Read, Write};
use std::collections::VecDeque;
use crate::character::Character;
use crate::map::{Map, RoomTable};
use crate::packs::{
    IdleChoices,
    IdlePack,
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
    Wildcard,
    Idle,
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
    pub timestamp_hours: u8,
    pub timestamp_minutes: u8,
    pub event_type: EventType,
    pub idle_pack: Option<IdlePack>,
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
    pub game_over_pack: Option<GameOverPack>,
    pub wildcard_line: String
}

impl<'a, 'b, 'c> Event<'a, 'b, 'c> {
    pub fn introduction() -> Self {
        Self {
            timestamp_hours: 0,
            timestamp_minutes: 0,
            event_type: EventType::Wildcard,
            idle_pack: None,
            movement_pack: None,
            trust_pack: None,
            dialogue_pack: None,
            ability_pack: None,
            murder_pack: None,
            corpse_discovery_pack: None,
            trial_start_pack: None,
            trial_voting_pack: None,
            trial_execution_pack: None,
            victory_pack: None,
            game_over_pack: None,
            wildcard_line: "This is an experimental detective text adventure.\n
            You are stuck in a building with nine people, and there's a murderer between them.\n
            For each round, which lasts for 30 in-game minutes, you'll be able to move 
            between rooms and talk to people, and try and gain their trust.\n
            If you gain someone's trust, you'll acquire a new ability associated with that person, 
            some of them are passive, some active, but all of them are helpful in order to
            solve the mistery.\n
            You can also check people's profiles before engaging in conversation with them.\n
            During the day, if someone (including you) spends one hour alone with the killer in the same room, 
            this person dies. If you die, it's game over. If someone else dies, though, 
            after the corpse is discovered by you, a trial 
            will happen, and you'll vote in order to execute someone. People who trust in you are likely to vote for 
            the same as you, and people will vote for those who they have the least affinity with.\n
            During night, you need to be extra careful, because if someone meets the killer, they're 
            instantly dead.\n
            You have the option to stay locked in your room during night and sleep, and you'll be safe, 
            but someone always dies during the night, so, be prepared to look for a corpse after dawn.\n\n
            Simply put, talk to people to build trust, use the abilities wisely and your logical thinking 
            to find out who's the killer, and don't get killed yourself.\n
            There's two ways to lose this game: either be the only one remaining besides the killer, or get killed. 
            You win when you get the real killer executed and sleep a peaceful night.\n
            ".to_string()
        }
    }

    pub fn start(map: Map, characters: Vec<Character>) -> Self {
        Self {
            timestamp_hours: 7,
            timestamp_minutes: 0,
            event_type: EventType::Idle,
            idle_pack: Some(IdlePack::starter(map, characters)),
            movement_pack: None,
            trust_pack: None,
            dialogue_pack: None,
            ability_pack: None,
            murder_pack: None,
            corpse_discovery_pack: None,
            trial_start_pack: None,
            trial_voting_pack: None,
            trial_execution_pack: None,
            victory_pack: None,
            game_over_pack: None,
            wildcard_line: String::new()
        }
    }

    pub fn execute(&self, map: &mut Map, characters: Vec<Character>, protag: &mut Protag) -> Vec<Event<'a, 'b, 'c>> {
        match self.event_type {
            EventType::Wildcard => {
                println!("{}", self.wildcard_line);
                vec![]
            }

            EventType::Idle => {
                println!("{}", format!("It is currently {}:{:02}.", self.timestamp_hours, self.timestamp_minutes));
                println!("{}", format!("You are now in the {}.", RoomTable[protag.location as usize]));
                match map.chars_in_rooms[protag.location as usize].len() {
                    0 => println!("There's nobody else here."),
                    1 => println!("{}", format!("You are alone here with {}.", map.chars_in_rooms[protag.location as usize][0])),
                    _ => {
                        for i in 0..map.chars_in_rooms[protag.location as usize].len() {
                            print!("{}", map.chars_in_rooms[protag.location as usize][i]);
                            if i < map.chars_in_rooms[protag.location as usize].len() - 2 {
                                print!(", ");
                            }
                            else if i == map.chars_in_rooms[protag.location as usize].len() - 2 {
                                print!(" and ");
                            }
                        }
                        print!(" are also here with you.\n");
                    }
                }

                // display actions
                println!("What will you do? Type and confirm your action's number.");
                match &self.idle_pack {
                    Some(idle_pack) => {
                        let mut i = 1;
                        for choice in &idle_pack.choices {
                            println!("{}-> {}", i, choice);
                            i += 1;
                        }
                    },
                    None => {
                        Event::event_without_pack_panic();
                    }
                }

                // get input
                let mut buffer = String::new();
                loop {
                    stdin().read_line(&mut buffer).expect("Couldn't read input from stdin.");
                    let parse_result = buffer.trim().parse::<u8>();
                    println!("{}", buffer);
                    match parse_result {
                        Ok(i) => {
                            if i > 0 && i <= self.idle_pack.as_ref().unwrap().choices.len() as u8 {
                                break
                            }
                            else {
                                print!("Please input a valid index. ");
                            }
                        },
                        Err(..) => {
                            print!("Please input an acceptable integer. ");
                        }
                    }
                }
                let choice = buffer.trim().parse::<u8>().unwrap() - 1;
                match self.idle_pack.as_ref().unwrap().events[choice as usize] {
                    IdleChoices::ExamineCharacter => {
                        return vec![
                            Event {
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::Wildcard,
                                idle_pack: None,
                                movement_pack: None,
                                trust_pack: None,
                                dialogue_pack: None,
                                ability_pack: None,
                                murder_pack: None,
                                corpse_discovery_pack: None,
                                trial_start_pack: None,
                                trial_voting_pack: None,
                                trial_execution_pack: None,
                                victory_pack: None,
                                game_over_pack: None,
                                wildcard_line: characters[choice as usize / 2].details.clone()
                            },
                            Event {
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::Idle,
                                idle_pack: self.idle_pack.clone(),
                                movement_pack: None,
                                trust_pack: None,
                                dialogue_pack: None,
                                ability_pack: None,
                                murder_pack: None,
                                corpse_discovery_pack: None,
                                trial_start_pack: None,
                                trial_voting_pack: None,
                                trial_execution_pack: None,
                                victory_pack: None,
                                game_over_pack: None,
                                wildcard_line: characters[choice as usize].details.clone()
                            }
                        ]
                    },
                    IdleChoices::TalkToCharacter => {
                        return vec![];
                    },
                    IdleChoices::MoveRoom => {
                        return vec![];
                    }
                }
            }

            _ => {
                vec![]
            }
        }
    }

    pub fn event_without_pack_panic() {
        panic!("ERROR: an event from a given type was queued without an event pack from the same type.\n 
        This was not supposed to happen.\n
        Please contact the main dev about this.\n");
    }
}

pub struct EventQueue<'a, 'b, 'c> {
    pub events: VecDeque<Event<'a, 'b, 'c>>,
    pub characters: Vec<Character>,
    pub map: Map,
    pub protag: Protag
}

impl<'a, 'b, 'c>  EventQueue<'a, 'b, 'c>  {
    pub fn new() -> Self {
        let mut events = VecDeque::new();
        events.push_back(Event::introduction());
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
        events.push_back(Event::start(map.clone(), characters.clone()));
        let protag = Protag::new();

        Self {
            events,
            characters,
            map,
            protag
        }
    }

    pub fn execute_event(&mut self) {
        // here we execute all char actions
        let consequences = self.events[0].execute(&mut self.map, self.characters.clone(), &mut self.protag);
        for event in consequences {
            self.events.push_back(event);
        }
    }

    pub fn allow_next_event(&self) -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        let mut stdin = stdin();

        write!(stdout, "Press Enter to continue... ")?;
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
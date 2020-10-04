use std::io::{stdin, stdout, Read, Write};
use std::collections::VecDeque;
use num_traits::FromPrimitive;

use crate::character::{Character, NUMBER_OF_CHARS};
use crate::map::{Map, RoomType, RoomTable, NUMBER_OF_ROOMS};
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
    pub movement_pack: Option<MovementPack>,
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
            During nighttime (22:00 to 6:30), you need to be extra careful, because if someone meets the killer, they're 
            instantly dead.\n
            You have the option to stay locked in your room during nighttime and sleep, and you'll be safe, 
            but someone always dies during the night, so, be prepared to look for a corpse after dawn.\n\n
            Simply put, talk to people to build trust, use the abilities wisely and your logical thinking 
            to find out who's the killer, and don't get killed yourself.\n
            There's two ways to lose this game: either be the only one remaining besides the killer, or get killed. 
            You win when you get the real killer executed and sleep a peaceful night.\n
            ".to_string()
        }
    }

    pub fn start(game_data: &GameData) -> Self {
        Self {
            timestamp_hours: 7,
            timestamp_minutes: 0,
            event_type: EventType::Idle,
            idle_pack: Some(IdlePack::starter(&game_data.map, &game_data.characters)),
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

    pub fn execute(&mut self, game_data: &mut GameData) -> Vec<Event<'a, 'b, 'c>> {
        match self.event_type {
            EventType::Wildcard => {
                println!("{}", self.wildcard_line);
                let mut stdout = stdout();
                let mut stdin = stdin();

                write!(stdout, "Press Enter to continue... ").unwrap();
                stdout.flush().unwrap();

                    // Read a single byte and discard
                let mut buffer = [0u8];
                let _ = stdin.read(&mut buffer).unwrap();

                vec![]
            }

            EventType::Idle => {
                println!("{}", format!("It is currently {}:{:02}.", self.timestamp_hours, self.timestamp_minutes));
                println!("{}", format!("You are now in the {}.", RoomTable[game_data.protag.location as usize]));
                match game_data.map.chars_in_rooms[game_data.protag.location as usize].len() {
                    0 => println!("There's nobody else here."),
                    1 => println!("{}", format!("You are alone here with {}.", game_data.map.chars_in_rooms[game_data.protag.location as usize][0])),
                    _ => {
                        for i in 0..game_data.map.chars_in_rooms[game_data.protag.location as usize].len() {
                            print!("{}", game_data.map.chars_in_rooms[game_data.protag.location as usize][i]);
                            if i < game_data.map.chars_in_rooms[game_data.protag.location as usize].len() - 2 {
                                print!(", ");
                            }
                            else if i == game_data.map.chars_in_rooms[game_data.protag.location as usize].len() - 2 {
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
                let pack = self.idle_pack.as_ref().unwrap();
                match pack.events[choice as usize] {
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
                                wildcard_line: game_data.characters[pack.chars_indices[choice as usize]].details.clone()
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
                                wildcard_line: String::new()
                            }
                        ]
                    },
                    IdleChoices::TalkToCharacter => {
                        return vec![];
                    },
                    IdleChoices::MoveRoom => {
                        let location = game_data.protag.location;
                        let chars_in_rooms = game_data.map.chars_in_rooms.clone();
                        return vec![
                            Event {
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::Movement,
                                idle_pack: None,
                                movement_pack: Some(MovementPack{
                                    protag_moves: true,
                                    move_origin: location,
                                    move_index: FromPrimitive::from_usize(pack.room_indices[choice as usize]).unwrap(),
                                    character_index: 0,

                                }),
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
                        ];
                    }
                }
            },

            EventType::Dialogue => {
                vec![]
            },

            EventType::TrustGain => {
                vec![]
            },

            EventType::TrustFail => {
                vec![]
            },

            EventType::Movement => {
                self.movement(&mut game_data.map, &mut game_data.characters, &mut game_data.protag);
                self.update_timestamps();
                let mut choices = vec![];
                let mut events = vec![];
                let mut chars_indices = vec![];
                let mut room_indices = vec![];
                self.create_choices(&mut game_data.map, &mut game_data.characters, &mut game_data.protag, &mut choices, &mut events, &mut chars_indices, &mut room_indices);

                let pack = self.movement_pack.as_mut().unwrap();
                if !pack.protag_moves {
                    return vec![];
                }

                vec![
                    Event {
                        timestamp_hours: self.timestamp_hours,
                        timestamp_minutes: self.timestamp_minutes,
                        event_type: EventType::Idle,
                        idle_pack: Some(IdlePack {
                            choices,
                            events,
                            chars_indices,
                            room_indices
                        }),
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
                ]
            },

            EventType::Ability => {
                vec![]
            },

            EventType::Murder => {
                vec![]
            },

            EventType::TrialStart => {
                vec![]
            },

            EventType::TrialVoting => {
                vec![]
            },

            EventType::TrialExecution => {
                vec![]
            },

            EventType::Victory => {
                vec![]
            },

            EventType::GameOver => {
                vec![]
            }
        }
    }

    pub fn movement(&mut self, map: &mut Box<Map>, characters: &mut Vec<Character>, protag: &mut Box<Protag>) {
        let pack = self.movement_pack.as_ref().unwrap();
        if pack.protag_moves {
            protag.location = pack.move_index
        }
        else {
            let index = map.chars_in_rooms[pack.move_origin as usize]
                .iter().position(|x| *x == characters[pack.character_index].name).unwrap();
            map.chars_in_rooms[pack.move_origin as usize].remove(index);
            map.chars_in_rooms[pack.move_index as usize].push(characters[pack.character_index].name.clone());
        }
    }

    pub fn create_choices(&mut self,
        map: &mut Box<Map>,
        characters: &mut Vec<Character>,
        protag: &mut Box<Protag>,
        choices: &mut Vec<String>,
        events: &mut Vec<IdleChoices>,
        chars_indices: &mut Vec<usize>,
        room_indices: &mut Vec<usize>) {
        let pack = self.movement_pack.as_ref().unwrap();
        for i in 0..NUMBER_OF_CHARS {
            if map.chars_in_rooms[pack.move_index as usize].contains(&characters[i].name) && characters[i].is_alive {
                chars_indices.push(i);
                room_indices.push(i);
                choices.push(format!("{}{}", "Spend some time with ", characters[i].name));
                events.push(IdleChoices::TalkToCharacter);
                choices.push(format!("{}{}{}", "Examine ", characters[i].name, "'s profile"));
                events.push(IdleChoices::ExamineCharacter);
            }
        }
        for i in 0..NUMBER_OF_ROOMS {
            if map.adjacency[protag.location as usize][i] == 1 {
                chars_indices.push(i);
                room_indices.push(i);
                events.push(IdleChoices::MoveRoom);
                choices.push(format!("Go to the {}", RoomTable[i]));
            }
        }
    }

    pub fn update_timestamps(&mut self) {
        self.timestamp_minutes += 30;
        if self.timestamp_minutes >= 60 {
            self.timestamp_minutes = 0;
            self.timestamp_hours += 1;
        }
    }

    pub fn event_without_pack_panic() {
        panic!("ERROR: an event from a given type was queued without an event pack from the same type.\n 
        This was not supposed to happen.\n
        Please contact the main dev about this.\n");
    }
}

pub struct EventQueue<'a, 'b, 'c> {
    pub events: VecDeque<Event<'a, 'b, 'c>>
}

impl<'a, 'b, 'c>  EventQueue<'a, 'b, 'c>  {
    pub fn new(game_data: &GameData) -> Self {
        let mut events = VecDeque::new();
        events.push_back(Event::introduction());
        events.push_back(Event::start(game_data));

        Self {
            events
        }
    }

    pub fn execute_event(&mut self, game_data: &mut GameData) {
        // here we execute all char actions
        let consequences = self.events[0].execute(game_data);
        for event in consequences {
            self.events.push_back(event);
        }
    }

    pub fn allow_next_event(&self) -> Result<(), std::io::Error> {
        // let mut stdout = stdout();
        // let mut stdin = stdin();

        // write!(stdout, "Press Enter to continue... ")?;
        // stdout.flush()?;

        //     // Read a single byte and discard
        // let mut buffer = [0u8];
        // let _ = stdin.read(&mut buffer).unwrap();

        print!("{}[2J", 27 as char);
        Ok(())
    }
    
    pub fn poll_next_event(&mut self) -> Option<Event> {
        self.events.pop_front()
    }
}

pub struct GameData {
    pub characters: Vec<Character>,
    pub map: Box<Map>,
    pub protag: Box<Protag>
}

impl GameData {
    pub fn new() -> Self {
        let map = Box::new(Map::new());
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
        let protag = Box::new(Protag::new());
        Self {
            map,
            characters,
            protag
        }
    }
}
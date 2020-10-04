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
    AbilityPack
};
use crate::protag::Protag;

pub enum EventType {
    Wildcard,
    Idle,
    TrustGain,
    Movement,
    Ability,
    CorpseDiscovery,
    TrialStart,
    TrialVoting,
    TrialExecution,
    Victory,
    GameOver
}

pub struct Event {
    pub timestamp_hours: u8,
    pub timestamp_minutes: u8,
    pub event_type: EventType,
    pub idle_pack: Option<IdlePack>,
    pub movement_pack: Option<MovementPack>,
    pub trust_pack: Option<TrustPack>,
    pub ability_pack: Option<AbilityPack>,
    pub wildcard_line: String
}

impl Event {
    pub fn introduction() -> Self {
        Self {
            timestamp_hours: 0,
            timestamp_minutes: 0,
            event_type: EventType::Wildcard,
            idle_pack: None,
            movement_pack: None,
            trust_pack: None,
            ability_pack: None,
            wildcard_line: "This is an experimental detective text adventure.\n
            You are stuck in a building with nine people, and there's a murderer between them.\n
            For each round, which lasts for 30 in-game minutes, you'll be able to move 
            between rooms and talk to people, and try and gain their trust.\n
            If you gain someone's trust, you'll acquire a new ability associated with that person, 
            some of them are passive, some active, but all of them are helpful in order to
            solve the mistery.\n
            You can also check people's profiles before engaging in conversation with them.\n
            During the day, if someone (including you) spends half an hour alone with the killer in the same room, 
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

    pub fn start() -> Self {
        Self {
            timestamp_hours: 7,
            timestamp_minutes: 0,
            event_type: EventType::Idle,
            idle_pack: Some(IdlePack{}),
            movement_pack: None,
            trust_pack: None,
            ability_pack: None,
            wildcard_line: String::new()
        }
    }

    pub fn execute(&mut self, game_data: &mut GameData) -> Vec<Event> {
        match self.event_type {
            EventType::Wildcard => {
                println!("{}", self.wildcard_line);
                self.wait_enter();

                vec![]
            }

            EventType::Idle => {
                println!("{}", format!("It is currently {}:{:02}.", self.timestamp_hours, self.timestamp_minutes));

                if self.timestamp_hours == 7 && self.timestamp_minutes == 0 {
                    for i in 0..game_data.map.chars_in_rooms.len() {
                        game_data.map.chars_in_rooms[i].clear();
                    }
                    for character in game_data.characters.clone() {
                        if character.is_alive {
                            game_data.map.chars_in_rooms[RoomType::MainHall as usize].push(character.name);
                        }
                    }
                }
                else if self.timestamp_hours == 21 {
                    println!("It's almost nighttime. You may want to go back to your room.");
                }
                else if self.timestamp_hours >= 22 || self.timestamp_hours <= 6 {
                    println!("Therefore, it's nighttime. Be careful.");
                    if self.timestamp_hours == 22 && self.timestamp_minutes == 0 {
                        // send almost everyone to their rooms
                        let mut random_pick = rand::random::<usize>() % NUMBER_OF_CHARS;
                        while !game_data.characters[random_pick].is_alive || game_data.characters[random_pick].is_killer {
                            random_pick += 1;
                            random_pick %= NUMBER_OF_CHARS;
                        }
                        for i in 0..game_data.map.chars_in_rooms.len() {
                            for j in 0..game_data.map.chars_in_rooms[i].len() {
                                if game_data.map.chars_in_rooms[i][j] != game_data.killer_name 
                                    && game_data.map.chars_in_rooms[i][j] != game_data.characters[random_pick].name {
                                    // remove name
                                    game_data.map.chars_in_rooms[i].remove(j);
                                }
                            }
                        }
                    }
                }

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

                let mut choices = vec![];
                let mut events = vec![];
                let mut chars_indices = vec![];
                let mut room_indices = vec![];

                self.create_choices(&mut game_data.map,
                    &mut game_data.characters,
                    &mut game_data.protag,
                    &mut choices,
                    &mut events,
                    &mut chars_indices,
                    &mut room_indices,
                    self.timestamp_hours);

                // display actions
                println!("What will you do? Type and confirm your action's number.");
                let mut i = 1;
                for choice in &choices {
                    println!("{}-> {}", i, choice);
                    i += 1;
                }

                // get input
                let mut buffer = String::new();
                loop {
                    stdin().read_line(&mut buffer).expect("Couldn't read input from stdin.");
                    let parse_result = buffer.trim().parse::<u8>();
                    println!("{}", buffer);
                    match parse_result {
                        Ok(i) => {
                            if i > 0 && i <= choices.len() as u8 {
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
                match events[choice as usize] {
                    IdleChoices::ExamineCharacter => {
                        return vec![
                            Event {
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::Wildcard,
                                idle_pack: None,
                                movement_pack: None,
                                trust_pack: None,
                                ability_pack: None,
                                wildcard_line: game_data.characters[chars_indices[choice as usize]].details.clone()
                            },
                            Event {
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::Idle,
                                idle_pack: self.idle_pack.clone(),
                                movement_pack: None,
                                trust_pack: None,
                                ability_pack: None,
                                wildcard_line: String::new()
                            }
                        ]
                    },
                    IdleChoices::TalkToCharacter => {
                        game_data.protag.moved = false;
                        self.check_murder(&mut game_data.map, &mut game_data.characters);
                        if self.check_game_over(&mut game_data.map, &mut game_data.characters, &mut game_data.protag, self.timestamp_hours) {
                            return vec![Event{
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::GameOver,
                                idle_pack: None,
                                movement_pack: None,
                                trust_pack: None,
                                ability_pack: None,
                                wildcard_line: String::new()
                            }];
                        }
                        else {
                            return vec![Event{
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::TrustGain,
                                idle_pack: None,
                                movement_pack: None,
                                trust_pack: Some(TrustPack {
                                    character_index: chars_indices[choice as usize]
                                }),
                                ability_pack: None,
                                wildcard_line: String::new()
                            }];
                        }
                    },
                    IdleChoices::MoveRoom => {
                        game_data.protag.moved = true;
                        self.check_murder(&mut game_data.map, &mut game_data.characters);
                        if self.check_game_over(&mut game_data.map, &mut game_data.characters, &mut game_data.protag, self.timestamp_hours) {
                            return vec![Event{
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::GameOver,
                                idle_pack: None,
                                movement_pack: None,
                                trust_pack: None,
                                ability_pack: None,
                                wildcard_line: String::new()
                            }];
                        }

                        let location = game_data.protag.location;
                        let mut ret = vec![];
                        self.enqueue_npc_movement(game_data, &mut ret);
                        
                        ret.push(Event {
                            timestamp_hours: self.timestamp_hours,
                            timestamp_minutes: self.timestamp_minutes,
                            event_type: EventType::Movement,
                            idle_pack: None,
                            movement_pack: Some(MovementPack{
                                protag_moves: true,
                                move_origin: location,
                                move_index: FromPrimitive::from_usize(room_indices[choice as usize]).unwrap(),
                                character_index: 0,

                            }),
                            trust_pack: None,
                            ability_pack: None,
                            wildcard_line: String::new()
                        });

                        if game_data.map.has_corpse[room_indices[choice as usize]] != "" {
                            ret.push(Event {
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::CorpseDiscovery,
                                idle_pack: None,
                                movement_pack: None,
                                trust_pack: None,
                                ability_pack: None,
                                wildcard_line: String::new()
                            });
                        }

                        ret
                    },
                    IdleChoices::GoToRoom => {
                        game_data.protag.location = RoomType::MainHall;
                        let mut ret = vec![
                            Event {
                                timestamp_hours: self.timestamp_hours,
                                timestamp_minutes: self.timestamp_minutes,
                                event_type: EventType::Wildcard,
                                idle_pack: None,
                                movement_pack: None,
                                trust_pack: None,
                                ability_pack: None,
                                wildcard_line: "You went back to your room and locked the door.
                                You have a good night of sleep.\n
                                ".to_string()
                            },
                            Event {
                                timestamp_hours: 7,
                                timestamp_minutes: 0,
                                event_type: EventType::Wildcard,
                                idle_pack: None,
                                movement_pack: None,
                                trust_pack: None,
                                ability_pack: None,
                                wildcard_line: "Nighttime is now over, it's 7 o'clock.
                                You leave your room and prepare to meet the other people in the Main Hall.\n
                                ".to_string()
                            }
                        ];

                        if self.check_victory(&game_data.characters) {
                            ret.push(Event {
                                timestamp_hours: 7,
                                timestamp_minutes: 0,
                                event_type: EventType::Victory,
                                idle_pack: None,
                                movement_pack: None,
                                trust_pack: None,
                                ability_pack: None,
                                wildcard_line: String::new()
                            });
                        }
                        else {
                            self.random_murder(&mut game_data.map, &mut game_data.characters);
                            ret.push(Event {
                                timestamp_hours: 7,
                                timestamp_minutes: 0,
                                event_type: EventType::Idle,
                                idle_pack: Some(IdlePack{}),
                                movement_pack: None,
                                trust_pack: None,
                                ability_pack: None,
                                wildcard_line: String::new()
                            });
                        }
                        ret
                    }
                }
            },

            EventType::TrustGain => {
                self.update_timestamps();
                let pack = self.trust_pack.as_ref().unwrap();
                if game_data.protag.trust_table[pack.character_index] {
                    println!("You already built trust with this character, but you spend time with them anyway.")
                }
                else {
                    println!("{}", game_data.characters[pack.character_index].trust_line);
                    self.build_trust(&mut game_data.protag, pack.character_index);
                }

                self.wait_enter();

                let mut ret = vec![];
                self.enqueue_npc_movement(game_data, &mut ret);

                ret.push(Event {
                        timestamp_hours: self.timestamp_hours,
                        timestamp_minutes: self.timestamp_minutes,
                        event_type: EventType::Idle,
                        idle_pack: Some(IdlePack {}),
                        movement_pack: None,
                        trust_pack: None,
                        ability_pack: None,
                        wildcard_line: String::new()
                    });
                
                ret
            },

            EventType::Movement => {
                self.movement(&mut game_data.map, &mut game_data.characters, &mut game_data.protag);
                self.update_timestamps();

                let pack = self.movement_pack.as_mut().unwrap();
                if !pack.protag_moves {
                    return vec![];
                }

                vec![
                    Event {
                        timestamp_hours: self.timestamp_hours,
                        timestamp_minutes: self.timestamp_minutes,
                        event_type: EventType::Idle,
                        idle_pack: Some(IdlePack {}),
                        movement_pack: None,
                        trust_pack: None,
                        ability_pack: None,
                        wildcard_line: String::new()
                    }
                ]
            },

            EventType::Ability => {
                vec![]
            },

            EventType::CorpseDiscovery => {
                println!("A CORPSE HAS BEEN FOUND!");
                println!("You can't help but scream in despair as you find what remains of {} lying on the ground,
                after being brutally murdered in the {}.", game_data.map.has_corpse[game_data.protag.location as usize],
                RoomTable[game_data.protag.location as usize]);
                if game_data.protag.hindsight {
                    println!("Your hindsight also tells you that {} was killed from {}:00 to {}:00",
                    game_data.map.has_corpse[game_data.protag.location as usize],
                    game_data.map.murder_timestamp[game_data.protag.location as usize],
                    game_data.map.murder_timestamp[game_data.protag.location as usize] + 1 % 24);
                }
                println!("But still even after the shock, all the remaining survivors must go through the trial.");
                self.wait_enter();
                vec![
                    Event {
                        timestamp_hours: self.timestamp_hours,
                        timestamp_minutes: self.timestamp_minutes,
                        event_type: EventType::TrialStart,
                        idle_pack: None,
                        movement_pack: None,
                        trust_pack: None,
                        ability_pack: None,
                        wildcard_line: String::new()
                    }
                ]
            },

            EventType::TrialStart => {
                println!("The corpses were cleaned up and everyone is preparing for the trial.");
                println!("From all of the people remaining, one of them committed a terrible crime.");
                println!("Will the criminal go unpunished, or will the voters make the right choice?");
                print!("You see ");
                for i in 0..game_data.characters.len() {
                    if game_data.characters[i].is_alive {
                        print!("{}, ", game_data.characters[i].name);
                    }
                }
                print!("in a circle, with different expressions. 
                Tension starts to build up around the room.\n");
                if game_data.protag.investigation {
                    println!("Your investigation find out the last places each one has respectively been to: ");
                    for i in 0..game_data.characters.len() {
                        if game_data.characters[i].is_alive {
                            print!("{} has been to the {}, the {}, and the {}.", game_data.characters[i].name,
                            game_data.characters[i].last_positions[0],
                            game_data.characters[i].last_positions[1],
                            game_data.characters[i].last_positions[2]);
                        }
                    }
                }

                println!("Who will you vote for? Type your choice and confirm.");
                let mut index = 1;
                for i in 0..game_data.characters.len() {
                    if game_data.characters[i].is_alive {
                        println!("{} -> {}", index, game_data.characters[i].name);
                        index += 1;
                    }
                }
                vec![]
            },

            EventType::TrialVoting => {
                vec![]
            },

            EventType::TrialExecution => {
                vec![]
            },

            EventType::Victory => {
                println!("After a safe night, the remaining people find themselves reunited in the main hall.\n
                It seems that no murder attempt has happened tonight!\n
                Yay! You caught the real killer!");
                println!("Thanks for playing this game and well done.");
                self.wait_enter();
                vec![]
            },

            EventType::GameOver => {
                println!("Game over.");
                println!("You were the last remaining person of the group, or the killer managed to caught you 
                before you managed to solve this mistery, leading to you unavoidable death.");
                println!("You were murdered by {}.", game_data.killer_name);
                println!("Thanks for playing, and better luck next time, I guess.");
                self.wait_enter();
                vec![]
            }
        }
    }

    pub fn build_trust(&self, protag: &mut Box<Protag>, index: usize) {
        protag.trust_table[index] = true;
    }

    pub fn enqueue_npc_movement(&mut self, game_data: &mut GameData, ret: &mut Vec<Event>) {
        for i in 0..game_data.characters.len() {
            let self_char = game_data.characters[i].clone();
            match game_data.characters[i].ai.choose_movement(game_data.map.clone(), self_char, i) {
                Some(new_pack) => {
                    ret.push(Event {
                        timestamp_hours: self.timestamp_hours,
                        timestamp_minutes: self.timestamp_minutes,
                        event_type: EventType::Movement,
                        idle_pack: None,
                        movement_pack: Some(new_pack),
                        trust_pack: None,
                        ability_pack: None,
                        wildcard_line: String::new()
                    });
                },
                _ => {
                    ()
                }
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
            for i in 0..2 {
                characters[pack.character_index].last_positions[i] = characters[pack.character_index].last_positions[i+1];
            }
            characters[pack.character_index].last_positions[2] = RoomTable[pack.move_origin as usize];
        }
    }

    pub fn random_murder(&mut self, map: &mut Box<Map>, characters: &mut Vec<Character>) {
        use rand::random;
        let mut killed = random::<usize>() % NUMBER_OF_CHARS;
        while !characters[killed].is_alive || characters[killed].is_killer {
            killed += 1;
            killed %= NUMBER_OF_CHARS; 
        }
        characters[killed].is_alive = false;
        let room = random::<usize>() % NUMBER_OF_ROOMS;
        map.has_corpse[room] = characters[killed].name.clone();
        map.murder_timestamp[room] = random::<u8>() % 7;
        }

    pub fn check_murder(&self, map: &mut Box<Map>, characters: &mut Vec<Character>) {
        for i in 0..map.chars_in_rooms.len() {
            if map.chars_in_rooms[i].len() == 2 {
                let index0 = characters.iter().position(|x| x.name == map.chars_in_rooms[i][0]).unwrap();
                let index1 = characters.iter().position(|x| x.name == map.chars_in_rooms[i][1]).unwrap();
                if characters[index0].last_positions[2] == RoomTable[i] && characters[index1].last_positions[2] == RoomTable[i] {
                    if characters[index0].is_killer {
                        characters[index1].is_alive = false;
                        map.has_corpse[i] = characters[index1].name.clone();
                        map.murder_timestamp[i] = self.timestamp_hours;
                    }
                    else if characters[index1].is_killer {
                        characters[index0].is_alive = false;
                        map.has_corpse[i] = characters[index0].name.clone();
                        map.murder_timestamp[i] = self.timestamp_hours;
                    }
                }
            }
        }
    }

    pub fn check_game_over(&self, map: &mut Box<Map>, characters: &mut Vec<Character>, protag: &mut Box<Protag>, timestamp_hours: u8) -> bool {
        let mut alive_characters = 0;
        for character in characters.clone() {
            if character.is_alive {
                alive_characters += 1;
            }
        }
        if alive_characters == 1 {
            // only one left game over
            return true;
        }
        else if timestamp_hours >= 22 || timestamp_hours <= 6 {
            // nighttime game over
            for i in 0..map.chars_in_rooms[protag.location as usize].len() {
                let index = characters.iter().position(|x| x.name == map.chars_in_rooms[protag.location as usize][i]).unwrap();
                if characters[index].is_killer {
                    return true;
                }
            }
            return false;
        }
        else {
            // spending time with the killer
            if map.chars_in_rooms[protag.location as usize].len() == 1 {
                let index = characters.iter().position(|x| x.name == map.chars_in_rooms[protag.location as usize][0]).unwrap();
                if characters[index].is_killer && !protag.moved {
                    return true;
                }
            }
            return false;
        }
    }

    pub fn check_victory(&self, characters: &Vec<Character>) -> bool {
        for character in characters {
            if !character.is_alive && character.is_killer {
                return true;
            }
        }
        false
    }

    pub fn create_choices(&mut self,
        map: &mut Box<Map>,
        characters: &mut Vec<Character>,
        protag: &mut Box<Protag>,
        choices: &mut Vec<String>,
        events: &mut Vec<IdleChoices>,
        chars_indices: &mut Vec<usize>,
        room_indices: &mut Vec<usize>,
        timestamp_hours: u8) {

        for i in 0..NUMBER_OF_CHARS {
            if map.chars_in_rooms[protag.location as usize].contains(&characters[i].name) && characters[i].is_alive {
                chars_indices.push(i);
                room_indices.push(i);
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

        if timestamp_hours >= 21 || timestamp_hours <= 6 {
            events.push(IdleChoices::GoToRoom);
            choices.push("Go back to your room and sleep.".to_string());
        }
    }

    pub fn update_timestamps(&mut self) {
        self.timestamp_minutes += 30;
        if self.timestamp_minutes >= 60 {
            self.timestamp_minutes = 0;
            self.timestamp_hours += 1;
        }
        if self.timestamp_hours >= 24 {
            self.timestamp_hours = 0;
        }
    }

    pub fn wait_enter(&self) {
        let mut stdout = stdout();
        let mut stdin = stdin();

        write!(stdout, "Press Enter to continue... ").unwrap();
        stdout.flush().unwrap();

        // Read a single byte and discard
        let mut buffer = [0u8];
        let _ = stdin.read(&mut buffer).unwrap();
    }

    pub fn event_without_pack_panic() {
        panic!("ERROR: an event from a given type was queued without an event pack from the same type.\n 
        This was not supposed to happen.\n
        Please contact the main dev about this.\n");
    }
}

pub struct EventQueue {
    pub events: VecDeque<Event>
}

impl EventQueue {
    pub fn new() -> Self {
        let mut events = VecDeque::new();
        events.push_back(Event::introduction());
        events.push_back(Event::start());

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
    pub protag: Box<Protag>,
    pub killer_name: String
}

impl GameData {
    pub fn new() -> Self {
        let map = Box::new(Map::new());
        let mut characters = vec![
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

        let random_uint = rand::random::<usize>() % NUMBER_OF_CHARS;
        characters[random_uint].is_killer = true;
        let killer_name = characters[random_uint].name.clone();

        let protag = Box::new(Protag::new());
        Self {
            map,
            characters,
            protag,
            killer_name
        }
    }
}
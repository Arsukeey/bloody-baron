use crate::character::{Character, NUMBER_OF_CHARS};
use crate::map::{Map, NUMBER_OF_ROOMS};
use crate::packs::MovementPack;
use num_traits::FromPrimitive;

pub const STOP_BIAS: usize = 3;

#[derive(Clone)]
pub struct AI {
    pub affinity: [f32; NUMBER_OF_CHARS],
    pub movement_tendency: [[f32; NUMBER_OF_ROOMS]; NUMBER_OF_ROOMS]
}

impl AI {
    pub fn new(map: &Box<Map>) -> Self {
        let affinity = [1.0 / NUMBER_OF_CHARS as f32; NUMBER_OF_CHARS];
        let mut movement_tendency = [[0.0; NUMBER_OF_ROOMS]; NUMBER_OF_ROOMS];

        // generate initial movement setup
        for i in 0..NUMBER_OF_ROOMS {
            let mut adjacent_rooms = Vec::new();
            for j in 0..NUMBER_OF_ROOMS {
                if map.adjacency[i][j] == 1 {
                    adjacent_rooms.push(j);
                }
            }
            for j in 0..adjacent_rooms.len() {
                movement_tendency[i][adjacent_rooms[j]] += 1.0 / (NUMBER_OF_ROOMS + STOP_BIAS) as f32;
            }
        }

        Self {
            affinity,
            movement_tendency
        }
    }

    pub fn choose_movement(&mut self, map: Box<Map>, character: Character, index: usize) -> Option<MovementPack> {
        if !character.is_alive {
            return None;
        }

        let move_origin = map.chars_in_rooms.iter().
            position(|x| x.contains(&character.name)).unwrap();
        let mut random_float = rand::random::<f32>();
        let mut move_index: i32 = -1;
        for i in 0..self.movement_tendency[move_origin].len() {
            random_float -= self.movement_tendency[move_origin][i];
            if random_float <= 0.0 {
                move_index = i as i32;
                break;
            }
        }
        match move_index {
            -1 => None,
            _ => Some(MovementPack {
                protag_moves: false,
                character_index: index,
                move_origin: FromPrimitive::from_usize(move_origin).unwrap(),
                move_index: FromPrimitive::from_i32(move_index).unwrap()
            })
        }
    }
}
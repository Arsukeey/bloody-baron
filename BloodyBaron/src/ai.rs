use crate::character::{NUMBER_OF_CHARS};
use crate::map::{Map, NUMBER_OF_ROOMS};

#[derive(Clone)]
pub struct AI {
    pub affinity: [f32; NUMBER_OF_CHARS],
    pub movement_tendency: [[f32; NUMBER_OF_ROOMS]; NUMBER_OF_ROOMS]
}

impl AI {
    pub fn new(map: &Map) -> Self {
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
                movement_tendency[i][adjacent_rooms[j]] += 1.0 / NUMBER_OF_ROOMS as f32;
            }
        }

        Self {
            affinity,
            movement_tendency
        }
    }
}
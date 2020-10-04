use rand::random;
use num_derive::FromPrimitive;    

pub const NUMBER_OF_ROOMS: usize = 8;

pub const RoomTable: [&str; NUMBER_OF_ROOMS+1] = [
    "Main Hall",
    "Bathroom",
    "Kitchen",
    "Gym",
    "Inn Hallway",
    "Library",
    "Trash Room",
    "Shed",
    "Bedroom"
];

#[derive(Copy, Clone, FromPrimitive)]
pub enum RoomType {
    MainHall,
    Bathroom,
    Kitchen,
    Gym,
    InnHallway,
    Library,
    TrashRoom,
    Shed
}

#[derive(Debug, Clone)]
pub struct Map {
    pub chars_in_rooms: Vec<Vec<String>>,
    pub adjacency: [[u8; NUMBER_OF_ROOMS]; NUMBER_OF_ROOMS],
    pub has_corpse: [String; NUMBER_OF_ROOMS],
    pub murder_timestamp: [u8; NUMBER_OF_ROOMS]
}

impl Map {
    pub fn new() -> Self {
        let chars_in_rooms = vec![
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![],
            vec![]
        ];
        let adjacency = Map::get_adjacency();
        let has_corpse: [String; NUMBER_OF_ROOMS] = Default::default();
        let murder_timestamp = [0; NUMBER_OF_ROOMS];
        Self {
            chars_in_rooms,
            adjacency,
            has_corpse,
            murder_timestamp
        }
    }

    fn get_adjacency() -> [[u8; NUMBER_OF_ROOMS]; NUMBER_OF_ROOMS] {
        let mut adjacency = [[0; NUMBER_OF_ROOMS]; NUMBER_OF_ROOMS];

        for i in 0..NUMBER_OF_ROOMS {
            let number_of_adjacent_rooms = random::<u8>() % NUMBER_OF_ROOMS as u8;
            let mut list_of_adjacent: Vec<u8> = vec![];
            for _ in 0..number_of_adjacent_rooms {
                let mut chosen_room = random::<u8>() % NUMBER_OF_ROOMS as u8;
                while chosen_room as usize == i || list_of_adjacent.contains(&chosen_room)  {
                    chosen_room += 1;
                    chosen_room %= NUMBER_OF_ROOMS as u8;
                }
                list_of_adjacent.push(chosen_room);
            }

            for index in list_of_adjacent {
                adjacency[i][index as usize] = 1;
                adjacency[index as usize][i] = 1;
            }
        }

        adjacency
    }
}
use rand::random;

pub const NUMBER_OF_ROOMS: usize = 9;

pub enum RoomType {
    MainHall,
    Bathroom,
    Kitchen,
    Gym,
    InnHallway,
    Library,
    TrashRoom,
    Shed,
    SleepingRoom
}

#[derive(Debug)]
pub struct Map {
    pub chars_in_rooms: Vec<Vec<String>>,
    pub adjacency: [[u8; NUMBER_OF_ROOMS]; NUMBER_OF_ROOMS]
}

impl Map {
    pub fn new() -> Self {
        let chars_in_rooms = vec![
            vec![
                "Freya".to_string(),
                "Ravi".to_string(),
                "Luna".to_string(),
                "John".to_string(),
                "Amanda".to_string(),
                "Vincent".to_string(),
                "Larissa".to_string(),
                "Odette".to_string(),
                "Chio".to_string()
            ],
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
        Self {
            chars_in_rooms,
            adjacency
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
                    println!("{}", chosen_room);
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
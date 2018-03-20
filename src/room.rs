use map;
use object;
use room_generator;

pub struct Room {
    pub rooms: Vec<object::Object>
}

impl Room {
    fn generate_room(map: &map::map::Map, strategy: &Fn(&map::map::Map) -> Vec<object::Object>) -> Vec<object::Object> {
        strategy(map)
    }

    pub fn new(map: &map::map::Map) -> Room {
        Room {
            rooms: Room::generate_room(map, &room_generator::grid::generate_room),
        }
    }
}

#[cfg(test)]
mod room_tests {
    use map::*;
    use room::*;

    #[test]
    fn test_new() {
        let map = Map::new(64, 32);
        let room = Room::new(&map);
        assert_eq!(1,2);
    }
}

use room;
use path_generator;
use object;

pub struct Path {
    pub paths: Vec<object::Object>
}

impl Path {
    fn generate_path(room: &room::Room, strategy: &Fn(&room::Room) -> Vec<object::Object>) -> Vec<object::Object> {
        strategy(room)
    }

    pub fn new(room: &room::Room) -> Path {
        Path {
            paths: Path::generate_path(room, &path_generator::simple::generate_path),
        }
    }
}

#[cfg(test)]
mod path_tests {
    use map::*;
    use room::*;
    // use path::*;

    #[test]
    fn test_new() {
        let map = Map::new(64, 32);
        let room = Room::new(map);
        // let path = Path::new(room);
        assert_eq!(1,2);
    }
}

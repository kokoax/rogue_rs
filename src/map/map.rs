use map;

pub struct Map {
    pub map:  Vec<Vec<String>>,
    pub info: map::map_info::MapInfo,
}

impl Map {
    fn generate_map(map_info: &map::map_info::MapInfo) -> Vec<Vec<String>> {
        let wall = "#";
        (0..(map_info.w)).into_iter().map(|_i| {
            (0..(map_info.h)).into_iter().map(|_j| {
                wall.to_string()
            }).collect()
        }).collect()
    }

    pub fn set_map(map: Vec<Vec<String>>) {
    }

    pub fn new(map: Vec<Vec<String>>) -> Map {
        Map {
            map: map,
            info: map::map_info::MapInfo::new(map.len() as i64, map[0].len() as i64),
        }
    }

    pub fn new(map_info: &map::map_info::MapInfo) -> Map {
        Map {
            map: Map::generate_map(map_info),
            info: map_info.clone(),
        }
    }
}

#[cfg(test)]
mod map_tests {
    use map::*;
    #[test]
    fn test_new() {
        let map = Map::new(64, 32);
        assert_eq!(map.w, 64);
        assert_eq!(map.h, 32);
    }
}

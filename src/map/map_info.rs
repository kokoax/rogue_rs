use std;

#[derive(Copy)]
pub struct MapInfo {
    pub w: i64,
    pub h: i64,
}

impl MapInfo {
    pub fn new(w: i64, h: i64) -> MapInfo {
        MapInfo {
            w: w,
            h: h,
        }
    }
}

impl std::clone::Clone for MapInfo {
    fn clone(&self) -> MapInfo { *self }
}

#[cfg(test)]
mod map_info_tests {
    use map::*;
    #[test]
    fn test_new() {
        let map = Map::new(64, 32);
        assert_eq!(map.w, 64);
        assert_eq!(map.h, 32);
    }
}

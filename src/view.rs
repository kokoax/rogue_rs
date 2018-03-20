use ncurses::*;

use map;
use room;
use path;
use object;

fn objet_view(o: &object::Object, c: &str) {
    for x in (o.x)..(o.x+o.w) {
        for y in (o.y)..(o.y+o.h) {
            mv(y as i32, x as i32);
            printw(c);
        }
    }
}

fn map_view(m: &map::map::Map) {
    objet_view(&object::Object::new(0,0,m.info.w,m.info.h), "#");
}

fn room_view(r: &room::Room) {
    for i in 0..(r.rooms.len()) {
        objet_view(&r.rooms[i], ".");
    }
}

fn path_view(p: &path::Path) {
    for i in 0..(p.paths.len()) {
        objet_view(&p.paths[i], "_");
    }
}

// fn all_view(m: Map, r: Room, p: Path, c: Creature, p: Player) {
pub fn all_view(m: &map::map::Map, r: &room::Room, p: &path::Path) {
    map_view(m);
    room_view(r);
    path_view(p);
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

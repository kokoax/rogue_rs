extern crate ncurses;
extern crate rand;

use ncurses::*;

mod view;
// mod creature;
mod map;
mod room;
mod room_generator;
mod path;
mod path_generator;
mod object;

const MAP_WIDTH:  i64 = 64;
const MAP_HEIGHT: i64 = 32;

/* setup ncurses */
fn init_ncurses() {
    initscr();
}

fn main() {
    init_ncurses();
    let mi = map::map_info::MapInfo::new(MAP_WIDTH,MAP_HEIGHT);
    let m = map::map::Map::new(&mi);
    let r = room::Room::new(&m);
    let p = path::Path::new(&r);
    view::all_view(&m, &r, &p);

    getch();
    endwin();
}

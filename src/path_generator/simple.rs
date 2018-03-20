use room;
use object;

fn four_direction_path(room: &room::Room) -> Vec<Vec<object::Object>>{
    let width = 8;
    let height = 2;
    (0..(room.rooms.len())).into_iter().map(|i| {
        vec![
            object::Object::new((room.rooms[i].x*2+room.rooms[i].w)/2, room.rooms[i].y-height,                 1, height),
            object::Object::new(room.rooms[i].x-width, (room.rooms[i].y*2+room.rooms[i].h)/2,                 width, 1),
            object::Object::new((room.rooms[i].x*2+room.rooms[i].w)/2, room.rooms[i].y+room.rooms[i].h, 1, height),
            object::Object::new(room.rooms[i].x+room.rooms[i].w, (room.rooms[i].y*2+room.rooms[i].h)/2, width, 1),
        ]
    }).collect()
}

pub fn generate_path(room: &room::Room) -> Vec<object::Object> {
    four_direction_path(&room).into_iter().flat_map(|item| {
        item
    }).collect()
}

#[cfg(test)]
mod simple_tests {
}

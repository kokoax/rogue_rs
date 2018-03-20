use map;
use object;

fn get_grid_room(x: i64, y: i64, num: i64) -> Vec<Vec<object::Object>> {
    let w = 6;
    let h = 6;
    (0..num).into_iter().map(|i| {
        (0..num).into_iter().map(|j| {
            object::Object::new(i*x + (x-w)/2, j*y + (y-h)/2, w, h)
        }).collect()
    }).collect()
}

pub fn generate_room(map: &map::map::Map) -> Vec<object::Object> {
    let num = 3;
    let x = map.info.w/num;
    let y = map.info.h/num;
    get_grid_room(x, y, num).into_iter().flat_map(|item| {
        item
    }).collect()
}

#[cfg(test)]
mod grid_tests {
}

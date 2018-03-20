pub struct Object {
    pub x: i64,
    pub y: i64,
    pub w: i64,
    pub h: i64,
}

impl Object {
    pub fn new(x: i64, y: i64, w: i64, h: i64) -> Object {
        Object {
            x: x,
            y: y,
            w: w,
            h: h,
        }
    }
}

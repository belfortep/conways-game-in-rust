#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    pub x_position: i32,
    pub y_position: i32,
}

impl Point {
    pub fn new(x_position: i32, y_position: i32) -> Self {
        Point {
            x_position,
            y_position,
        }
    }
}

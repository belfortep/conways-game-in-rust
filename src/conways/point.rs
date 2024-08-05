use std::collections::HashSet;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x_position: i32,
    y_position: i32,
}

impl Point {
    pub fn new(x_position: i32, y_position: i32) -> Self {
        Point {
            x_position,
            y_position,
        }
    }

    pub fn neighbours(&self) -> HashSet<Point> {
        let mut neighbours = HashSet::new();

        neighbours.insert(Point::new(self.x_position + 1, self.y_position));
        neighbours.insert(Point::new(self.x_position - 1, self.y_position));
        neighbours.insert(Point::new(self.x_position, self.y_position + 1));
        neighbours.insert(Point::new(self.x_position, self.y_position - 1));
        neighbours.insert(Point::new(self.x_position + 1, self.y_position + 1));
        neighbours.insert(Point::new(self.x_position + 1, self.y_position - 1));
        neighbours.insert(Point::new(self.x_position - 1, self.y_position + 1));
        neighbours.insert(Point::new(self.x_position - 1, self.y_position - 1));

        neighbours
    }
}

use rand::Rng;
use std::{collections::HashSet, ops::Range};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Default)]
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

    pub fn generate_random_point(range: Range<i32>) -> Point {
        let x_position = rand::thread_rng().gen_range(range.clone());
        let y_position = rand::thread_rng().gen_range(range);

        Point::new(x_position, y_position)
    }
}

impl From<&str> for Point {
    fn from(s: &str) -> Self {
        let mut positions = s.split(',');

        let (Some(x_position), Some(y_position), None) =
            (positions.next(), positions.next(), positions.next())
        else {
            return Self::default();
        };

        let Ok(x_position) = x_position.parse() else {
            return Self::default();
        };

        let Ok(y_position) = y_position.parse() else {
            return Self::default();
        };

        Self {
            x_position,
            y_position,
        }
    }
}

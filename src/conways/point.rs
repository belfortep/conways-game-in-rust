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

impl TryFrom<&str> for Point {
    type Error = &'static str;
    fn try_from(s: &str) -> Result<Self, &'static str> {
        let mut positions = s.split(',');

        let (Some(x_position), Some(y_position), None) =
            (positions.next(), positions.next(), positions.next())
        else {
            return Err("Bad format creating a Point, should be x_position,y_position");
        };

        let Ok(x_position) = x_position.parse() else {
            return Err("The x_position should be an integer");
        };

        let Ok(y_position) = y_position.parse() else {
            return Err("The y_position should be an integer");
        };

        Ok(Self {
            x_position,
            y_position,
        })
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn can_create_a_point_from_string() {
        let point = Point::try_from("5,5").unwrap();

        assert_eq!(point.x_position, 5);
        assert_eq!(point.y_position, 5);
    }

    #[test]
    fn can_not_create_point_from_missing_position() {
        let point = Point::try_from("5");

        assert!(point.is_err())
    }

    #[test]
    fn can_not_create_point_from_wrong_type() {
        let point = Point::try_from("hi,5");

        assert!(point.is_err())
    }
}

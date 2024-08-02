use super::point::Point;

pub struct ConwaysGame {
    cell: Point,
}

impl ConwaysGame {
    pub fn new(cell: Point) -> Self {
        Self { cell }
    }

    pub fn next_generation(&self) {}

    pub fn is_alive(&self, cell: Point) -> bool {
        false
    }
}

#[cfg(test)]
mod test {

    use crate::conways_game::point::Point;

    use super::*;

    #[test]
    fn test_001_a_cell_without_neighbours_die_after_one_generation() {
        let cell = Point::new(0, 0);
        let conways_game = ConwaysGame::new(cell);

        conways_game.next_generation();

        assert!(!conways_game.is_alive(Point::new(0, 0)));
    }
}

use std::collections::HashSet;

use super::point::Point;

pub struct ConwaysGame {
    cells: HashSet<Point>,
}

impl ConwaysGame {
    pub fn new(cells: Vec<Point>) -> Self {
        Self {
            cells: HashSet::from_iter(cells),
        }
    }

    pub fn next_generation(&mut self) {
        let mut cells = HashSet::new();

        for cell in &self.cells {
            if self
                .cells
                .contains(&Point::new(cell.x_position, cell.y_position + 1))
                && self
                    .cells
                    .contains(&Point::new(cell.x_position, cell.y_position - 1))
            {
                cells.insert(cell.clone());
            }
        }
        self.cells = cells;
        
    }

    pub fn is_alive(&self, cell: Point) -> bool {
        self.cells.contains(&cell)
    }
}

#[cfg(test)]
mod test {

    use crate::conways_game::point::Point;

    use super::*;

    #[test]
    fn test_001_a_cell_without_neighbours_die_after_one_generation() {
        let cells = vec![Point::new(0, 0)];
        let mut conways_game = ConwaysGame::new(cells);

        conways_game.next_generation();

        assert!(!conways_game.is_alive(Point::new(0, 0)));
    }

    #[test]
    fn test_002_a_cell_with_two_neighbours_is_alive_after_one_generation() {
        let cells = vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, -1)];
        let mut conways_game = ConwaysGame::new(cells);

        conways_game.next_generation();

        assert!(conways_game.is_alive(Point::new(0, 0)));
    }
}

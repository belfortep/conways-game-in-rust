use std::collections::HashSet;

use super::point::Point;

pub struct ConwaysGame {
    alive_cells: HashSet<Point>,
}

impl ConwaysGame {
    pub fn new(alive_cells: Vec<Point>) -> Self {
        Self {
            alive_cells: HashSet::from_iter(alive_cells),
        }
    }

    pub fn next_generation(&mut self) {
        let alive_cells = self.alive_cells();

        let resurrected_cells = self.resurrected_cells();

        let mut cells = HashSet::new();

        for cell in alive_cells.union(&resurrected_cells) {
            cells.insert(*cell);
        }

        self.alive_cells = cells;
    }

    pub fn is_alive(&self, cell: Point) -> bool {
        self.alive_cells.contains(&cell)
    }

    pub fn cells_do<F: FnMut(&Point)>(&self, mut closure: F) {
        for cell in &self.alive_cells {
            closure(cell);
        }
    }

    fn resurrected_cells(&self) -> HashSet<Point> {
        let mut cells = HashSet::new();

        self.cells_do(| cell | {
            for neigbour in cell.neighbours() {
                if self.can_resurrect(&neigbour) {
                    cells.insert(neigbour);
                }
            }
        });

        cells
    }

    fn alive_cells(&self) -> HashSet<Point> {
        let mut cells = HashSet::new();

        self.cells_do(|cell| {
            if self.can_survive(cell) {
                cells.insert(*cell);
            }
        });

        cells
    }

    fn can_resurrect(&self, cell: &Point) -> bool {
        let ammount_of_neighbours = cell.neighbours().intersection(&self.alive_cells).count();

        ammount_of_neighbours == 3
    }

    fn can_survive(&self, cell: &Point) -> bool {
        let ammount_of_neighbours = cell.neighbours().intersection(&self.alive_cells).count();

        ammount_of_neighbours == 2 || ammount_of_neighbours == 3
    }
}

#[cfg(test)]
mod test {

    use crate::conways::point::Point;

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

    #[test]
    fn test_003_a_cell_with_three_neighbours_is_alive_after_one_generation() {
        let cells = vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(-1, 0),
            Point::new(1, 0),
        ];
        let mut conways_game = ConwaysGame::new(cells);

        conways_game.next_generation();

        assert!(conways_game.is_alive(Point::new(0, 0)));
    }

    #[test]
    fn test_004_a_cell_with_more_than_three_neighbours_dies_after_one_generation() {
        let cells = vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(-1, 0),
            Point::new(1, 0),
            Point::new(1, 1),
        ];
        let mut conways_game = ConwaysGame::new(cells);

        conways_game.next_generation();

        assert!(!conways_game.is_alive(Point::new(0, 0)));
    }

    #[test]
    fn test_005_a_dead_cell_with_exactly_three_neighbours_resurrects_after_one_generation() {
        let cells = vec![Point::new(0, 1), Point::new(-1, 0), Point::new(1, 0)];
        let mut conways_game = ConwaysGame::new(cells);

        conways_game.next_generation();

        assert!(conways_game.is_alive(Point::new(0, 0)));
    }
}

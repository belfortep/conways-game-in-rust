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
        let alive_cells = self.alive_cells();

        let resurrected_cells = self.resurrected_cells();

        let mut cells = HashSet::new();

        for cell in alive_cells.union(&resurrected_cells) {
            cells.insert(cell.clone());
        }

        self.cells = cells;
    }

    pub fn is_alive(&self, cell: Point) -> bool {
        self.cells.contains(&cell)
    }

    fn resurrected_cells(&self) -> HashSet<Point> {
        let mut cells = HashSet::new();

        for cell in &self.cells {
            for neigbour in cell.neighbours() {
                if self.can_resurrect(&neigbour) {
                    cells.insert(neigbour);
                }
            }
        }

        cells
    }

    fn alive_cells(&self) -> HashSet<Point> {
        let mut cells = HashSet::new();

        for cell in &self.cells {
            if self.can_survive(cell) {
                cells.insert(cell.clone());
            }

        }
        cells
    }

    fn can_resurrect(&self, cell: &Point) -> bool {
        let ammount_of_neighbours = cell.neighbours().intersection(&self.cells).count();

        ammount_of_neighbours == 3
    }

    fn can_survive(&self, cell: &Point) -> bool {
        let ammount_of_neighbours = cell.neighbours().intersection(&self.cells).count();
        
        ammount_of_neighbours == 2 || ammount_of_neighbours == 3
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

    #[test]
    fn test_003_a_cell_with_three_neighbours_is_alive_after_one_generation() {
        let cells = vec![Point::new(0, 0), Point::new(0, 1), Point::new(-1, 0), Point::new(1, 0)];
        let mut conways_game = ConwaysGame::new(cells);

        conways_game.next_generation();

        assert!(conways_game.is_alive(Point::new(0, 0)));
    }

    
    #[test]
    fn test_004_a_cell_with_more_than_three_neighbours_dies_after_one_generation() {
        let cells = vec![Point::new(0, 0), Point::new(0, 1), Point::new(-1, 0), Point::new(1, 0), Point::new(1,1)];
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

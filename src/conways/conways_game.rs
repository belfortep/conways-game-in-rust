use std::collections::HashSet;

use super::point::Point;

pub struct ConwaysGame {
    alive_cells: HashSet<Point>,
    height: u32,
    width: u32,
}

impl ConwaysGame {
    pub fn new(alive_cells: Vec<Point>, height: u32, width: u32) -> Result<Self, String> {
        for cell in &alive_cells {
            if !Self::cell_is_in_range(cell, height, width) {
                return Err(format!(
                    "A cell was not in the specified range, game width: {}, game height: {}, cell x: {}, cell y: {}",
                    width, height, cell.x_position, cell.y_position
                ));
            }
        }

        Ok(Self {
            alive_cells: HashSet::from_iter(alive_cells),
            height,
            width,
        })
    }

    fn cell_is_in_range(cell: &Point, height: u32, width: u32) -> bool {
        (0 <= cell.x_position && cell.x_position < width as i32)
            && (0 <= cell.y_position && cell.y_position < height as i32)
    }

    pub fn next_generation(&mut self) {
        let mut cells = HashSet::new();

        self.all_cells_do(|cell| {
            if self.can_resurrect(cell) {
                cells.insert(*cell);
            }
            if self.can_survive(cell) && self.alive_cells.contains(cell) {
                cells.insert(*cell);
            }
        });
        self.alive_cells = cells;
    }

    pub fn is_alive(&self, cell: Point) -> bool {
        self.alive_cells.contains(&cell)
    }

    pub fn all_cells_do<F: FnMut(&Point)>(&self, mut closure: F) {
        for x in 0..self.width {
            for y in 0..self.height {
                closure(&Point::new(x as i32, y as i32));
            }
        }
    }

    pub fn add_cells(&mut self, cells_to_add: Vec<Point>) {
        for cell in &cells_to_add {
            if Self::cell_is_in_range(cell, self.height, self.width) {
                self.alive_cells.insert(*cell);
            }
        }
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
    fn a_cell_without_neighbours_die_after_one_generation() {
        let cells = vec![Point::new(0, 0)];
        let mut conways_game = ConwaysGame::new(cells, 10, 10).unwrap();

        conways_game.next_generation();

        assert!(!conways_game.is_alive(Point::new(0, 0)));
    }

    #[test]
    fn a_cell_with_two_neighbours_is_alive_after_one_generation() {
        let cells = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];
        let mut conways_game = ConwaysGame::new(cells, 10, 10).unwrap();

        conways_game.next_generation();

        assert!(conways_game.is_alive(Point::new(0, 0)));
    }

    #[test]
    fn a_cell_with_three_neighbours_is_alive_after_one_generation() {
        let cells = vec![
            Point::new(0, 0),
            Point::new(0, 1),
            Point::new(1, 1),
            Point::new(1, 0),
        ];
        let mut conways_game = ConwaysGame::new(cells, 10, 10).unwrap();

        conways_game.next_generation();

        assert!(conways_game.is_alive(Point::new(0, 0)));
    }

    #[test]
    fn a_cell_with_more_than_three_neighbours_dies_after_one_generation() {
        let cells = vec![
            Point::new(1, 1),
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(0, 1),
            Point::new(1, 2),
        ];
        let mut conways_game = ConwaysGame::new(cells, 10, 10).unwrap();

        conways_game.next_generation();

        assert!(!conways_game.is_alive(Point::new(1, 1)));
    }

    #[test]
    fn a_dead_cell_with_exactly_three_neighbours_resurrects_after_one_generation() {
        let cells = vec![Point::new(0, 1), Point::new(1, 1), Point::new(1, 0)];
        let mut conways_game = ConwaysGame::new(cells, 10, 10).unwrap();

        conways_game.next_generation();

        assert!(conways_game.is_alive(Point::new(0, 0)));
    }

    #[test]
    fn can_add_one_cell_after_a_generation_passed() {
        let cells = vec![Point::new(0, 0)];
        let mut conways_game = ConwaysGame::new(cells, 10, 10).unwrap();

        conways_game.next_generation();

        assert!(!conways_game.is_alive(Point::new(0, 0)));

        let cells = vec![Point::new(0, 0)];
        conways_game.add_cells(cells);

        assert!(conways_game.is_alive(Point::new(0, 0)));
    }

    #[test]
    fn can_add_multiple_cells_after_a_generation_passed() {
        let cells = vec![Point::new(0, 0)];
        let mut conways_game = ConwaysGame::new(cells, 10, 10).unwrap();

        conways_game.next_generation();

        assert!(!conways_game.is_alive(Point::new(0, 0)));

        let cells = vec![Point::new(0, 0), Point::new(0, 1), Point::new(1, 0)];
        conways_game.add_cells(cells);
        conways_game.next_generation();

        assert!(conways_game.is_alive(Point::new(0, 0)));
    }

    #[test]
    fn can_not_have_cells_outside_bounds() {
        let cells = vec![Point::new(20, 0)];
        let conways_game = ConwaysGame::new(cells, 10, 10);

        assert!(conways_game.is_err());
    }

    #[test]
    fn an_not_add_cells_outside_bounds() {
        let cells = vec![Point::new(0, 0)];
        let mut conways_game = ConwaysGame::new(cells, 10, 10).unwrap();
        let cells = vec![Point::new(1, 0), Point::new(50, 0)];
        conways_game.add_cells(cells);
        assert!(conways_game.is_alive(Point::new(1, 0)));
        assert!(!conways_game.is_alive(Point::new(50, 0)));
    }
}

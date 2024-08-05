use std::{thread::sleep, time::Duration};

use macroquad::{color::{BLACK, WHITE}, shapes::draw_rectangle, window::{clear_background, next_frame, screen_height, screen_width}};

use super::conways_game::ConwaysGame;

pub struct ConwaysGameView {
    conways_game: ConwaysGame,
    is_drawing: bool
}

impl ConwaysGameView {
    pub fn new(conways_game: ConwaysGame) -> Self {
        Self {
            conways_game,
            is_drawing: true
        }
    }

    pub async fn start_drawing(&mut self) {
        while self.is_drawing {
            clear_background(WHITE);

            self.conways_game.next_generation();

            self.conways_game.cells_do(|cell| {
                draw_rectangle(screen_width()/2.0 - (cell.x_position * 5) as f32, screen_height()/2.0 - (cell.y_position * 5) as f32, 10.0, 10.0, BLACK);
            });

            sleep(Duration::from_millis(200));

            next_frame().await;
        }
    }

}

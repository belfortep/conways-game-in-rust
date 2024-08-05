use std::{thread::sleep, time::Duration};

use macroquad::{
    color::{BLACK, WHITE},
    shapes::draw_rectangle,
    window::{clear_background, next_frame, screen_height, screen_width},
};

use super::{
    conways_game::ConwaysGame,
    conways_game_constants::{CELLS_HEIGHT, CELLS_WIDTH, CONSTANT_WAIT, VIEW_SCALE_FACTOR},
};

pub struct ConwaysGameView {
    conways_game: ConwaysGame,
    is_drawing: bool,
}

impl ConwaysGameView {
    pub fn new(conways_game: ConwaysGame) -> Self {
        Self {
            conways_game,
            is_drawing: true,
        }
    }

    pub async fn start_drawing(&mut self) {
        while self.is_drawing {
            clear_background(WHITE);

            self.conways_game.next_generation();

            self.conways_game.cells_do(|cell| {
                let width = CELLS_WIDTH;
                let height = CELLS_HEIGHT;
                let scale_factor = VIEW_SCALE_FACTOR;
                let x_position = Self::get_x_position(cell.x_position, scale_factor);
                let y_position = Self::get_y_position(cell.y_position, scale_factor);
                draw_rectangle(x_position, y_position, width, height, BLACK);
            });

            sleep(Duration::from_secs_f32(CONSTANT_WAIT));

            next_frame().await;
        }
    }

    fn get_x_position(position: i32, scale_factor: i32) -> f32 {
        screen_width() / 2.0 - (position * scale_factor) as f32
    }

    fn get_y_position(position: i32, scale_factor: i32) -> f32 {
        screen_height() / 2.0 - (position * scale_factor) as f32
    }
}

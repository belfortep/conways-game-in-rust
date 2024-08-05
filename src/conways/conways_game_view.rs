

use macroquad::{
    color::{BLACK, WHITE}, shapes::draw_rectangle, time::get_time, window::{clear_background, next_frame, screen_height, screen_width}
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

        let last_frame_time = get_time();

        while self.is_drawing {

            let current_frame_time = get_time();

            if current_frame_time - last_frame_time >= CONSTANT_WAIT {
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
            }
            
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

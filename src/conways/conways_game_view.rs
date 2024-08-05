use std::{thread::sleep, time::Duration};

use macroquad::prelude::*;


use super::{
    conways_game::ConwaysGame,
    conways_game_constants::{CELLS_HEIGHT, CELLS_WIDTH, CONSTANT_WAIT, VIEW_SCALE_FACTOR},
};

pub struct ConwaysGameView {
    conways_game: ConwaysGame,
    is_passing_generations: bool,
    generations_count: u32
}

impl ConwaysGameView {
    pub fn new(conways_game: ConwaysGame) -> Self {
        Self {
            conways_game,
            is_passing_generations: true,
            generations_count: 0
        }
    }

    pub async fn start_drawing(&mut self) {
        loop {
            self.verify_input();

            clear_background(WHITE);

            if self.is_passing_generations {
                self.next_generation();
            }

            self.draw_cells();

            draw_text(format!("Generation number {}", self.generations_count).as_str(), 10.0, 20.0 ,20.0, BLACK);

            sleep(Duration::from_secs_f32(CONSTANT_WAIT));

            next_frame().await;
        }
    }

    fn draw_cells(&mut self) {
        self.conways_game.cells_do(|cell| {
            let width = CELLS_WIDTH;
            let height = CELLS_HEIGHT;
            let scale_factor = VIEW_SCALE_FACTOR;
            let x_position = Self::get_x_position(cell.x_position, scale_factor);
            let y_position = Self::get_y_position(cell.y_position, scale_factor);
            draw_rectangle(x_position, y_position, width, height, BLACK);
        });
    }   

    fn next_generation(&mut self) {
        self.conways_game.next_generation();
        self.generations_count += 1;
    }


    fn verify_input(&mut self) {
        if is_key_down(KeyCode::P) {
            self.is_passing_generations = !self.is_passing_generations;
        }
        if is_key_down(KeyCode::Enter) {
            self.next_generation();
        }
        if is_key_down(KeyCode::Escape) {
            println!("Closing the app...");
            std::process::exit(0);
        }
        
    }

    fn get_x_position(position: i32, scale_factor: i32) -> f32 {
        screen_width() / 2.0 - (position * scale_factor) as f32
    }

    fn get_y_position(position: i32, scale_factor: i32) -> f32 {
        screen_height() / 2.0 - (position * scale_factor) as f32
    }
}

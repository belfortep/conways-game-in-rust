use std::{thread::sleep, time::Duration};

use macroquad::{
    prelude::*,
    ui::{
        hash, root_ui,
        widgets::{self},
    },
};

use crate::parse_random_arguments;

use super::{
    conways_game::ConwaysGame,
    conways_game_constants::{CELLS_HEIGHT, CELLS_WIDTH, CONSTANT_WAIT, VIEW_SCALE_FACTOR},
    point::Point,
};

pub struct ConwaysGameView {
    conways_game: ConwaysGame,
    is_passing_generations: bool,
    generations_count: u32,
    random_state: String,
}

impl ConwaysGameView {
    pub fn new(conways_game: ConwaysGame) -> Self {
        Self {
            conways_game,
            is_passing_generations: true,
            generations_count: 0,
            random_state: String::new(),
        }
    }

    pub async fn start_drawing(&mut self) {
        loop {
            self.verify_input();
            self.verify_mouse_touch();

            clear_background(WHITE);

            if self.is_passing_generations {
                self.next_generation();
            }

            self.draw_cells();

            widgets::Window::new(hash!(), vec2(10.0, 10.), vec2(500., 60.))
                .label("Random state")
                .ui(&mut root_ui(), |ui| {
                    ui.input_text(
                        hash!(),
                        "<- format: max,min,ammount",
                        &mut self.random_state,
                    );
                });

            draw_text(
                format!("Generation number {}", self.generations_count).as_str(),
                520.0,
                30.0,
                20.0,
                BLACK,
            );

            sleep(Duration::from_secs_f32(CONSTANT_WAIT));

            next_frame().await;
        }
    }

    fn draw_cells(&mut self) {
        self.conways_game.cells_do(|cell| {
            let width = CELLS_WIDTH;
            let height = CELLS_HEIGHT;
            let scale_factor = VIEW_SCALE_FACTOR;
            let x_position =
                Self::convert_x_position_from_conways_unit_to_pixels(cell.x_position, scale_factor);
            let y_position =
                Self::convert_y_position_from_conways_unit_to_pixels(cell.y_position, scale_factor);
            draw_rectangle(x_position, y_position, width, height, BLACK);
        });
    }

    fn next_generation(&mut self) {
        self.conways_game.next_generation();
        self.generations_count += 1;
    }

    fn verify_input(&mut self) {
        if is_key_released(KeyCode::P) {
            self.is_passing_generations = !self.is_passing_generations;
        }
        if is_key_down(KeyCode::Up) {
            self.next_generation();
        }

        if is_key_released(KeyCode::Escape) {
            println!("Closing the app..");
            std::process::exit(0);
        }

        if is_key_released(KeyCode::Enter) {
            if let Ok(points) = parse_random_arguments(self.random_state.clone()) {
                let conways_game = ConwaysGame::new(points);
                self.conways_game = conways_game;
                self.generations_count = 0;
            }
        }
    }

    fn verify_mouse_touch(&mut self) {
        let mut cells_to_add = Vec::new();

        if is_mouse_button_down(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            let scale_factor = VIEW_SCALE_FACTOR as f32;
            let x_position =
                Self::convert_x_position_from_pixels_to_conways_unit(mouse_x, scale_factor);
            let y_position =
                Self::convert_y_position_from_pixels_to_conways_unit(mouse_y, scale_factor);

            cells_to_add.push(Point::new(x_position, y_position));
        }

        self.conways_game.add_cells(cells_to_add);
    }

    fn convert_x_position_from_conways_unit_to_pixels(position: i32, scale_factor: i32) -> f32 {
        screen_width() / 2.0 - (position * scale_factor) as f32
    }

    fn convert_y_position_from_conways_unit_to_pixels(position: i32, scale_factor: i32) -> f32 {
        screen_height() / 2.0 - (position * scale_factor) as f32
    }

    fn convert_x_position_from_pixels_to_conways_unit(position: f32, scale_factor: f32) -> i32 {
        (-1.0 * (position - screen_width() / 2.0) / scale_factor).round() as i32
    }

    fn convert_y_position_from_pixels_to_conways_unit(position: f32, scale_factor: f32) -> i32 {
        (-1.0 * (position - screen_height() / 2.0) / scale_factor).round() as i32
    }
}

use macroquad::{
    color::{BLACK, WHITE},
    prelude::*,
    shapes::draw_rectangle,
    window::{clear_background, next_frame},
};

use super::{
    conways_game::ConwaysGame,
    conways_game_constants::{
        CELLS_HEIGHT, CELLS_WIDTH, FPS, PADDING_X, PADDING_Y, VIEW_SCALE_FACTOR,
    },
    point::Point,
};

pub struct ConwaysGameView {
    conways_game: ConwaysGame,
    is_passing_generations: bool,
    generations_count: u32,
    fps: u32,
}

impl ConwaysGameView {
    pub fn new(conways_game: ConwaysGame) -> Self {
        Self {
            conways_game,
            is_passing_generations: false,
            generations_count: 0,
            fps: FPS,
        }
    }

    pub async fn start_drawing(&mut self) {
        let mut timer = 0.0;
        loop {
            self.verify_input();
            self.verify_mouse_touch();
            let constant_wait = 1.0 / self.fps as f32;

            timer += get_frame_time();
            if timer >= constant_wait && self.is_passing_generations {
                self.next_generation();
                timer = 0.0;
            }
            clear_background(WHITE);
            self.draw();
            next_frame().await;
        }
    }

    fn draw(&mut self) {
        draw_text(
            format!(
                "Generation number {}, FPS {}",
                self.generations_count, self.fps
            )
            .as_str(),
            10.0,
            30.0,
            20.0,
            BLACK,
        );

        draw_text("P to un/pause, Enter to pass one generation, Up key to increase speed, Down key to decrease speed, Escape to exit", 10.0, 50.0, 15.0, BLACK);

        self.conways_game.all_cells_do(|cell| {
            let x_position = Self::convert_x_position_from_conways_unit_to_pixels(
                cell.x_position,
                VIEW_SCALE_FACTOR,
            ) + PADDING_X;
            let y_position = Self::convert_y_position_from_conways_unit_to_pixels(
                cell.y_position,
                VIEW_SCALE_FACTOR,
            ) - PADDING_Y;

            if self.conways_game.is_alive(*cell) {
                draw_rectangle(x_position, y_position, CELLS_WIDTH, CELLS_HEIGHT, BLACK);
            } else {
                draw_rectangle_lines(x_position, y_position, CELLS_WIDTH, CELLS_HEIGHT, 2.0, GRAY);
            }
        });
    }

    fn next_generation(&mut self) {
        self.conways_game.next_generation();
        self.generations_count += 1;
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

    fn verify_input(&mut self) {
        if is_key_released(KeyCode::P) {
            self.is_passing_generations = !self.is_passing_generations;
        }
        if is_key_released(KeyCode::Enter) && !self.is_passing_generations {
            self.next_generation();
        }

        if is_key_released(KeyCode::Up) {
            self.fps += 1;
        }

        if is_key_released(KeyCode::Down) && self.fps >= 2 {
            self.fps -= 1;
        }

        if is_key_released(KeyCode::Escape) {
            println!("Closing the app..");
            std::process::exit(0);
        }
    }

    fn convert_x_position_from_conways_unit_to_pixels(position: i32, scale_factor: i32) -> f32 {
        (position * scale_factor) as f32
    }

    fn convert_y_position_from_conways_unit_to_pixels(position: i32, scale_factor: i32) -> f32 {
        -1.0 * ((position * scale_factor) as f32 - screen_height())
    }

    fn convert_x_position_from_pixels_to_conways_unit(position: f32, scale_factor: f32) -> i32 {
        ((position - PADDING_X) / scale_factor).round() as i32
    }

    fn convert_y_position_from_pixels_to_conways_unit(position: f32, scale_factor: f32) -> i32 {
        ((screen_height() - position - PADDING_Y) / scale_factor).round() as i32
    }
}

use std::io::stdin;

use conways::{
    command_line_parser::parser::{
        parse_points_arguments, parse_random_arguments, receive_command_line_arguments,
    },
    conways::{
        conways_game::ConwaysGame,
        conways_game_constants::{HEIGHT, WIDTH},
        conways_game_view::ConwaysGameView,
    },
};

#[macroquad::main("Conway's Game")]
async fn main() -> Result<(), String> {
    let args = receive_command_line_arguments()?;

    let mut cells = Vec::new();

    if args.get_flag("points") {
        let mut points_input = String::new();
        let stdin = stdin();
        stdin
            .read_line(&mut points_input)
            .map_err(|error| error.to_string())?;
        cells = parse_points_arguments(points_input.trim().to_string())?;
    }

    if let Some(random) = args.get_one::<String>("random") {
        cells = parse_random_arguments(random.clone())?;
    }

    let conways_game = ConwaysGame::new(cells, HEIGHT, WIDTH)?;
    let mut conways_game_view = ConwaysGameView::new(conways_game);

    conways_game_view.start_drawing().await;
    Ok(())
}

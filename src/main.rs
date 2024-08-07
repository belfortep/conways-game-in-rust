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

    if let Some(points) = args.get_one::<String>("p") {
        cells = parse_points_arguments(points.clone())?;
    }

    if let Some(random) = args.get_one::<String>("r") {
        cells = parse_random_arguments(random.clone())?;
    }

    let conways_game = ConwaysGame::new(cells, HEIGHT, WIDTH)?;
    let mut conways_game_view = ConwaysGameView::new(conways_game);

    conways_game_view.start_drawing().await;
    Ok(())
}

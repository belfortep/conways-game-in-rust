use conways::{conways::{conways_game::ConwaysGame, conways_game_view::ConwaysGameView}, parse_arguments};

#[macroquad::main("BasicShapes")]
async fn main() -> Result<(), String> {

    let args: Vec<String> = std::env::args().skip(1).collect();
    let cells = parse_arguments(args)?;
    let conways_game = ConwaysGame::new(cells);
    let mut conways_game_view = ConwaysGameView::new(conways_game);

    conways_game_view.start_drawing().await;
    Ok(())
}

use conways::conways::{conways_game::ConwaysGame, conways_game_view::ConwaysGameView, point::Point};

#[macroquad::main("BasicShapes")]
async fn main() {
    let cells = vec![Point::new(0, 0), Point::new(1, 0), Point::new(-1, 0)];
    let conways_game = ConwaysGame::new(cells);
    let mut conways_game_view = ConwaysGameView::new(conways_game);

    conways_game_view.start_drawing().await;
}

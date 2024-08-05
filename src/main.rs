use clap::{Arg, Command};
use conways::{
    conways::{conways_game::ConwaysGame, conways_game_view::ConwaysGameView},
    parse_points_arguments, parse_random_arguments,
};

#[macroquad::main("Conway's Game")]
async fn main() -> Result<(), String> {
    let args = Command::new(" Conway's game of life")
        .arg(
            Arg::new("p")
                .short('p')
                .long("points")
                .value_name("Points")
                .help("Specify the points in the format 0,0:1,0:1,1:...."),
        )
        .arg(
            Arg::new("r")
                .short('r')
                .long("Random")
                .value_name("Random")
                .help("Specify the range in the format max_value,min_value,ammout_of_points"),
        )
        .after_help("Don't use -r and -p at the same time")
        .get_matches();

    if args.contains_id("p") && args.contains_id("r") {
        return Err("Don't use -r and -p at the same time".into());
    }

    let mut cells = Vec::new();

    if let Some(points) = args.get_one::<String>("p") {
        cells = parse_points_arguments(points.clone())?;
    }

    if let Some(random) = args.get_one::<String>("r") {
        cells = parse_random_arguments(random.clone())?;
    }

    let conways_game = ConwaysGame::new(cells);
    let mut conways_game_view = ConwaysGameView::new(conways_game);

    conways_game_view.start_drawing().await;
    Ok(())
}

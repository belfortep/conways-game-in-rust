# conways-game-in-rust
Implementation of Conway's Game of Life with Rust and Web Assembly


## Execution
- `make run FILE=file_name` to run the game, else use `make run` to use the example `points.txt` file.
- `cargo run 0,0:1,0:....` to pass the points yourself
- `make random RNG=max_value,min_value,amount_of_points` to run the game with a random state, by default it goes with 10,-10,1000


## Controls
- "⬆️" to pass one generation at a time
- "🅿️" to pause/unpause the automatic pass of generations
- "Escape" to exit the game
- "Mouse button left" to place new cells

# conways-game-in-rust
Implementation of Conway's Game of Life with Rust and Web Assembly


## Execution
- `make run FILE=file_name` to run the game where `file_name` contains the initial state of the game, else use `make run` to use the example `points.txt` file.
- `cargo run -- -p 0,0:1,0:....` to pass the points yourself
- `make random RNG=max_value,min_value,amount_of_points` to run the game with a random state, by default it goes with 30,10,1000
- `cargo run -- -r max_value,min_value,ammount_of_points` to run the game with a random state by yourself
- The different ways to run the program are, with a initial state you pass `-p` with a random initial state `-r` or just `cargo run` to run with no initial state

## Controls
- "Enter" to pass one generation at a time
- "P" to pause/unpause the automatic generations
- "Escape" to exit the game
- "Mouse button left" to place new cells
- "Up key" to increase the speed of the game
- "Down key" to reduce the speed of the game

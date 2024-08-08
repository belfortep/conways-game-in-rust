# conways-game-in-rust
Implementation of Conway's Game of Life with Rust and Web Assembly


## Execution
- `make run FILE=file_name` to run the game where `file_name` contains the initial state of the game, else use `make run` to use the example `points.txt` file.
- `cargo run -- -p` to pass the points yourself with stdin
- `make random RNG=max_value,min_value,amount_of_points` to run the game with a random state, by default it goes with 30,10,1000
- `cargo run -- -r max_value,min_value,ammount_of_points` to run the game with a random state by yourself
- The different ways to run the program are, `-p` with a initial state you pass,  `-r` with a random initial state  or just `cargo run` to run with no initial state. You can't pass `-p` and  `-r` at the same time

## Controls
- "Enter" to pass one generation at a time
- "P" to pause/unpause the automatic generations
- "Escape" to exit the game
- "Mouse button left" to place new cells
- "Up key" to increase the speed of the game
- "Down key" to reduce the speed of the game

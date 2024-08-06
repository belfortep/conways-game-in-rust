# conways-game-in-rust
Implementation of Conway's Game of Life with Rust and Web Assembly

To run with some selected points from a file, just do ´make run FILE=file_name´, if you don't provide any file, it will use the ´points.txt´ file as an example
Or you can type ´cargo run 0,0:1,0:....´ to pass the points yourself
Also, you can type ´make random RNG=max_value,min_value,amount_of_points´ to start the game with a random state, by default it goes with 10,-10,1000

## Controls
- "⬆️" to pass one generation at a time
- "🅿️" to pause/unpause the automatic pass of generations
- "Escape" to exit the game
- "Mouse button left" to place new cells

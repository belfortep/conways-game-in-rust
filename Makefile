FILE = points.txt
RNG = 30,10,100

run:
	cargo run -- -p < $(FILE)

random:
	cargo run -- -r $(RNG)

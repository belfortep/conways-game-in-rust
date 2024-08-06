FILE = points.txt
ARGS = $(shell cat $(FILE))
RNG = 10,-10,1000

run:
	cargo run -- -p $(ARGS)

random:
	cargo run -- -r $(RNG)

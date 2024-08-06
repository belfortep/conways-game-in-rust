FILE = points.txt
ARGS = $(shell cat $(FILE))
RNG = 30,10,100

run:
	cargo run -- -p $(ARGS)

random:
	cargo run -- -r $(RNG)

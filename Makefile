FILE = points.txt
ARGS = $(shell cat $(FILE))

run:
	cargo run $(ARGS)

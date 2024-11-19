all:
	cargo build

run:
	cargo run


test:
	cargo test -- --nocapture

.PHONY: all test


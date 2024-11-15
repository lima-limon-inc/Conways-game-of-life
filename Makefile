all:
	cargo build

test:
	cargo test -- --nocapture

.PHONY: all test


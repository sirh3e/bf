DATA_DIR      = ./data
PROGRAMS_DIR  = $(DATA_DIR)/programs
GENERATED_DIR = $(DATA_DIR)/generated

.PHONY: all
all: configure cargo-fmt cargo-test cargo-build-debug

.PHONY: release
release:
	cargo build --release

.PHONY: transpile
transpile: c-transpile rust-transpile

.PHONY: c-transpile
c-transpile:
	cargo run --release --bin c -- --program-file $(PROGRAMS_DIR)/mandelbrot.bf --output-file $(GENERATED_DIR)/mandelbrot.c

.PHONY: rust-transpile
rust-transpile:
	cargo run --release --bin rust -- --program-file $(PROGRAMS_DIR)/mandelbrot.bf --output-file $(GENERATED_DIR)/mandelbrot.rs

.PHONY: build-debug
cargo-build-debug:
	cargo build

.PHONY: cargo-fmt
cargo-fmt:
	cargo fmt

.PHONY: cargo-test
cargo-test:
	cargo test

.PHONY: configure
configure:
	chmod +x configure
	./configure

.PHONY: clean
clean:
	rm -vf ./data/generated/*
	cargo clean
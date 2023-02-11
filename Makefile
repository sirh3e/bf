DATA_DIR      = ./data
PROGRAMS_DIR  = $(DATA_DIR)/programs
GENERATED_DIR = $(DATA_DIR)/generated

PROGRAMS_INPUT     = $(wildcard $(PROGRAMS_DIR)/*.bf)
PROGRAMS_OUTPUT_C  = $(patsubst $(PROGRAMS_DIR)/%.bf,$(GENERATED_DIR)/%.c,$(PROGRAMS_INPUT))
PROGRAMS_OUTPUT_RS = $(patsubst $(PROGRAMS_DIR)/%.bf,$(GENERATED_DIR)/%.rs,$(PROGRAMS_INPUT))

.PHONY: all
all: configure cargo-fmt cargo-test cargo-build-debug

.PHONY: release
release:
	cargo build --release

.PHONY: transpile
transpile: c-transpile rust-transpile

.PHONY: c-transpile
c-transpile: $(PROGRAMS_INPUT)
	for file in $(PROGRAMS_INPUT) ; do \
		cargo run --release --bin c -- --program-file $${file} --output-directory $(GENERATED_DIR) ; \
  	done

.PHONY: rust-transpile
rust-transpile: $(PROGRAMS_INPUT)
	for file in $(PROGRAMS_INPUT) ; do \
		cargo run --release --bin rust -- --program-file $${file} --output-directory $(GENERATED_DIR) ; \
  	done

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
	rm -vf ./data/logs/*
	cargo clean

.PHONY: simon-configure
simon-configure: configure transpile
	gcc -O3 -o $(GENERATED_DIR)/mandelbrot.c.exe $(GENERATED_DIR)/mandelbrot.c
	rustc -O $(GENERATED_DIR)/mandelbrot.rs -C target-cpu=native -o $(GENERATED_DIR)/mandelbrot.rs.exe

.PHONY: simon-run
simon-run:
	chmod +x ./scripts/simon-run.sh
	./scripts/simon-run.sh

.PHONY: simon-export
simon-export:
	tar czf simon-benchmark.tar.gz --directory=./data/logs .
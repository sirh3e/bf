.PHONY: all
all: configure cargo-fmt cargo-test cargo-build-debug

.PHONY: release
release:
	cargo build --release

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
	rm -vf ./bin/*
	cargo clean
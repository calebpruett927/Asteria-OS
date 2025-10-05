# Simple helpers
.PHONY: run build
build:
	cargo build -p asteria-kernel --release

run: build
	./target/release/asteria-kernel

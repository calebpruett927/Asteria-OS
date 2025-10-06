# Asteria OS — Makefile (v0.3)
# Devhost-first: builds and runs the host binary today.
# When xtask emits a UEFI image at dist/asteria-uefi.img, `make qemu` just works.

SHELL := /usr/bin/env bash
.ONESHELL:
.SHELLFLAGS := -euo pipefail -c

# ---- Config ---------------------------------------------------------------
RUST_TOOLCHAIN ?= nightly-2025-09-20
KERNEL_CRATE   := asteria-kernel
BOOT_CRATE     := asteria-boot
IMG            := dist/asteria-uefi.img
CARGO          := cargo

# ---- Phony targets --------------------------------------------------------
.PHONY: help fmt clippy check build run image qemu hash toolchain clean distclean

help:
	@cat <<'EOF'
Targets:
  fmt         - cargo fmt check
  clippy      - cargo clippy (deny warnings)
  check       - type-check the workspace
  build       - build $(KERNEL_CRATE) (devhost, release)
  run         - run  $(KERNEL_CRATE) (devhost)
  image       - build UEFI image via xtask → $(IMG)  (stub now)
  qemu        - launch QEMU with UEFI image (uses qemu-run.sh)
  hash        - write SHA256 of manifests/REPRO_MANIFEST.json to dist/
  toolchain   - print active rustc/cargo versions (should be $(RUST_TOOLCHAIN))
  clean       - cargo clean
  distclean   - clean + remove dist/
EOF

fmt:
	$(CARGO) fmt --all -- --check

clippy:
	$(CARGO) clippy --workspace --all-targets -- -D warnings

check:
	$(CARGO) check --workspace

build: fmt clippy
	$(CARGO) build -p $(KERNEL_CRATE) --release

run: build
	./target/release/$(KERNEL_CRATE)

# --- UEFI path (wired later via xtask) -------------------------------------
image:
	@echo "==> Building UEFI image with xtask (expect: $(IMG))"
	$(CARGO) run -p xtask --release
	@if [[ ! -f "$(IMG)" ]]; then \
	  echo "!! xtask did not produce $(IMG). Wire xtask to create it."; \
	  exit 1; \
	fi
	@echo "==> UEFI image ready: $(IMG)"

qemu: image
	@chmod +x qemu-run.sh
	./qemu-run.sh --uefi --img "$(IMG)"

# --- Governance helpers ----------------------------------------------------
hash:
	@mkdir -p dist
	@sha256sum manifests/REPRO_MANIFEST.json | awk '{print $$1}' > dist/MANIFEST_SHA256.txt
	@echo "Wrote dist/MANIFEST_SHA256.txt"

toolchain:
	@echo "Active toolchain (want $(RUST_TOOLCHAIN)):"
	rustup show active-toolchain
	rustc -V
	cargo -V

clean:
	$(CARGO) clean

distclean: clean
	rm -rf dist

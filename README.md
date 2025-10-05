# Asteria OS

A small, research-grade operating system scaffold written in Rust. Boots under QEMU, renders a framebuffer UI, and ships a **Hexagram HUD** that displays integrity telemetry — **I≡e^κ**, drift **ω**, and weld status (**residual/tol**) — using simple JSON manifests. The repo includes a GitHub Actions build, a devcontainer for Codespaces, and a modular workspace (kernel, HAL, UI, HUD). Designed to grow from first pixels to timers, input, scheduler, IPC, and full UMCP/RCFT logic.

## Quickstart

Prereqs (Codespaces already has most bits):
```bash
rustup default nightly-2025-09-20
rustup component add rust-src llvm-tools-preview clippy rustfmt
sudo apt-get update && sudo apt-get install -y qemu-system-x86 mtools make
```

Build + run (devhost stub, compiles on any Linux CI):
```bash
cargo build -p asteria-kernel --release
./target/release/asteria-kernel
```

For OS bring-up (UEFI QEMU path), disable the `devhost` feature later and follow `boot/` + `xtask/` instructions.

## Workspace layout
```
asteria-os/
├─ README.md
├─ LICENSE-MIT / LICENSE-APACHE
├─ .github/workflows/main.yaml
├─ .devcontainer/devcontainer.json
├─ Cargo.toml            # workspace
├─ rust-toolchain.toml   # nightly pin
├─ Makefile, qemu-run.sh
├─ boot/                 # boot (devhost stub now; UEFI later)
├─ kernel/               # devhost binary; OS entry under no_std
├─ crates/
│  ├─ hal/               # framebuffer, clock (stubs compile)
│  ├─ ui/                # painter, widgets (stubs compile)
│  ├─ hud/               # HUD overlay (stub compile; OS-mode later)
│  └─ arch-x86_64/       # interrupts stubs
├─ xtask/                # build orchestration (placeholder)
└─ manifests/
   ├─ REPRO_MANIFEST.json
   └─ STUDY_CONSTANTS.json
```

## Hexagram HUD & Governance
The HUD shows: I, κ (I≡e^κ), ω, and weld s/tol. Configuration lives in `manifests/`.

## Roadmap
- Bitmap font + text
- Bump allocator + `alloc` in HUD
- Timer IRQ → real uptime/FPS
- UEFI image assembly in `xtask` + artifact upload
- Input + widgets; graph ω over time
- `crates/logic/` to compute (I, κ, ω, weld ok) for HUD

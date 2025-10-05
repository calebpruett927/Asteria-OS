# Asteria OS

A small, research-grade operating system scaffold written in Rust. Boots under QEMU, renders a framebuffer UI, and ships a **Hexagram HUD** that displays integrity telemetry — **I≡e^κ**, drift **ω**, and weld status (**residual/tol**) — using simple JSON manifests. The repo includes a GitHub Actions build, a devcontainer for Codespaces, and a modular workspace (kernel, HAL, UI, HUD). Designed to grow from first pixels to timers, input, scheduler, IPC, and full UMCP/RCFT logic.

<!-- Badges (optional) -->
[![build](https://github.com/<you>/asteria-os/actions/workflows/build.yml/badge.svg)](https://github.com/<you>/asteria-os/actions/workflows/build.yml)
![license: MIT/Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-informational)

---

## Features (current)
- Rust microkernel scaffold with QEMU bring-up
- Framebuffer renderer + immediate-mode UI
- Hexagram HUD overlay (I, κ, ω, fps, weld s/tol, seed, manifest hash)
- GitHub Actions CI and Codespaces devcontainer
- Governance manifests: `REPRO_MANIFEST.json`, `STUDY_CONSTANTS.json`

> Status: early bring-up. Text rendering, allocator, timer IRQs, and UEFI image packaging are first-mile tasks (see Roadmap).

---

## Quickstart

### Prerequisites
- Rust nightly with `rust-src` and `llvm-tools-preview`
- QEMU, `make`, `bash`

```bash
rustup default nightly-2025-09-20
rustup component add rust-src llvm-tools-preview clippy rustfmt
# Debian/Ubuntu:
sudo apt-get update && sudo apt-get install -y qemu-system-x86 mtools make
make run
asteria-os/
├─ README.md
├─ LICENSE-MIT / LICENSE-APACHE
├─ .github/workflows/build.yml
├─ .devcontainer/devcontainer.json
├─ Cargo.toml            # workspace
├─ rust-toolchain.toml   # nightly pin
├─ Makefile, qemu-run.sh
├─ boot/                 # boot image (UEFI via bootloader)
├─ kernel/               # entry, init, main loop (HUD call site)
├─ crates/
│  ├─ hal/               # framebuffer, clock
│  ├─ ui/                # painter, widgets, font
│  ├─ hud/               # Hexagram HUD overlay
│  └─ arch-x86_64/       # interrupts stubs
├─ xtask/                # image build orchestration
└─ manifests/
   ├─ REPRO_MANIFEST.json
   └─ STUDY_CONSTANTS.json
// manifests/REPRO_MANIFEST.json (example)
{
  "weld_id": "asteria-v0.1-init",
  "tol": 0.005,
  "residual": 0.002,
  "seed": 12648430,
  "manifest_sha256": "TBD"
}
// manifests/STUDY_CONSTANTS.json (example)
{
  "omega_gates": { "stable": 0.038, "collapse": 0.30 },
  "C_ref": 0.0407,
  "Ilow": 0.594,
  "notes": "Thresholds with 95% CIs once data lands."
}

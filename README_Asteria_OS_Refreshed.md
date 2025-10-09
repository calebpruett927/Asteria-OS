
# Asteria-OS

**Canonical microkernel with built-in HUD for real-time integrity telemetry (`I ≡ e^κ`, `ω`) and weld-based governance. Runs on QEMU/UEFI with reproducible builds via GitHub Actions.**

---

## 🔍 Overview

Asteria-OS is a research-grade operating system kernel designed for epistemic transparency and symbolic traceability. It includes a live HUD that outputs:

- Integrity Metric: `I ≡ e^κ` (epistemic integrity)
- Drift Metric: `ω` (reasoning curvature)
- Weld Status: Live governance boundary tracking

This system acts as a core execution and telemetry layer for RCFT/ULRC systems and symbolic reasoning environments.

---

## 🖥️ Demo (Quick Start)

```bash
git clone https://github.com/calebpruett927/Asteria-OS.git
cd Asteria-OS
cargo build
qemu-system-x86_64 -kernel target/x86_64-unknown-none/debug/asteria_os
```

> Requires: Rust nightly, QEMU, `bootimage`, `llvm-tools-preview`

---

## 🧠 Architecture Diagram

```text
    +---------------------------+
    |    Asteria-OS Kernel      |
    |---------------------------|
    | HAL | Logic | Scheduler   |
    +---------------------------+
              ↓
    +---------------------------+
    |     Integrity HUD (UI)    |
    |  - I ≡ e^κ, ω metrics     |
    |  - Weld status            |
    +---------------------------+
              ↓
    +---------------------------+
    |   JSON Manifest Output    |
    | (telemetry for auditors)  |
    +---------------------------+
```

---

## 🧰 Features

- Modular crates: kernel, HAL, UI, HUD
- QEMU/UEFI boot targets
- GitHub Actions CI/CD
- Canonical telemetry output (JSON)
- Reproducibility: manifest-anchored execution
- Multiple licenses (MIT/Apache-2.0)

---

## 📜 License

This project is dual-licensed under MIT and Apache 2.0.

---

## 🧾 Citation & Canon

All work is published under the pseudonym *Clement Paulus* (legal: *Caleb Pruitt*). See canonical works and foundational papers at:

- [Zenodo Archive](https://zenodo.org/records/16990995)
- [Academia.edu Profile](https://independent.academia.edu/ClementPaulus)
- [OpenAI GPT Canonical Interface](https://chat.openai.com/gpts)

---

## 💬 Questions?

Reach out via GitHub Issues or submit via the canonical descent validator (coming soon).

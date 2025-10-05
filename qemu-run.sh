#!/usr/bin/env bash
set -euo pipefail
# Placeholder run. When UEFI image exists, switch to -drive with ESP.
qemu-system-x86_64 -machine q35 -m 512M -serial stdio -display sdl \  -kernel target/x86_64-unknown-none/release/asteria-kernel || true

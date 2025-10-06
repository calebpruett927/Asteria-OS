#!/usr/bin/env bash
# Asteria OS — QEMU runner
# Supports UEFI images (OVMF). Headless + GDB toggles via flags/env.
set -euo pipefail

# --- Defaults (can be overridden via env or flags) ---
MODE="${MODE:-uefi}"                     # uefi | bios (bios not wired yet)
IMG="${IMG:-dist/asteria-uefi.img}"      # path to UEFI disk image (FAT ESP)
MEM="${MEM:-1024M}"
CPUS="${CPUS:-2}"
GRAPHICS="${GRAPHICS:-sdl}"              # sdl | none (none -> -nographic)
GDB="${GDB:-0}"                          # 1 => -S -s for gdb
ACCEL="${ACCEL:-auto}"                   # auto | kvm | tcg
MACHINE="${MACHINE:-q35}"

usage() {
  cat <<EOF
Usage: $0 [--img PATH] [--mem 1024M] [--cpus 2] [--nographic|--sdl] [--gdb]
            [--uefi] [--bios] [--machine q35] [--accel auto|kvm|tcg]

Env overrides: IMG, MEM, CPUS, GRAPHICS, GDB, MODE, ACCEL, MACHINE
Examples:
  $0 --uefi --img dist/asteria-uefi.img
  GDB=1 GRAPHICS=none $0
EOF
}

# --- Parse flags ---
while [[ $# -gt 0 ]]; do
  case "$1" in
    --img) IMG="$2"; shift 2;;
    --mem) MEM="$2"; shift 2;;
    --cpus) CPUS="$2"; shift 2;;
    --nographic) GRAPHICS="none"; shift;;
    --sdl) GRAPHICS="sdl"; shift;;
    --gdb) GDB=1; shift;;
    --uefi) MODE="uefi"; shift;;
    --bios) MODE="bios"; shift;;
    --machine) MACHINE="$2"; shift 2;;
    --accel) ACCEL="$2"; shift 2;;
    -h|--help) usage; exit 0;;
    *) echo "Unknown arg: $1"; usage; exit 2;;
  esac
done

# --- Helpers ---
has_cmd() { command -v "$1" >/dev/null 2>&1; }

find_ovmf() {
  # Common paths on Debian/Ubuntu/Fedora
  for p in \
    /usr/share/OVMF/OVMF_CODE.fd \
    /usr/share/ovmf/OVMF_CODE.fd \
    /usr/share/qemu/OVMF_CODE.fd \
    /usr/share/edk2/x64/OVMF_CODE.fd \
    /usr/share/edk2/ovmf/OVMF_CODE.fd
  do
    [[ -f "$p" ]] && { echo "$p"; return 0; }
  done
  return 1
}

find_ovmf_vars() {
  for p in \
    /usr/share/OVMF/OVMF_VARS.fd \
    /usr/share/ovmf/OVMF_VARS.fd \
    /usr/share/qemu/OVMF_VARS.fd \
    /usr/share/edk2/x64/OVMF_VARS.fd \
    /usr/share/edk2/ovmf/OVMF_VARS.fd
  do
    [[ -f "$p" ]] && { echo "$p"; return 0; }
  done
  return 1
}

ensure_img() {
  if [[ ! -f "$IMG" ]]; then
    echo "❗ No UEFI image at: $IMG"
    echo "   Build one with your xtask once wired, e.g.:  cargo run -p xtask --release"
    exit 1
  fi
}

accel_args=()
case "$ACCEL" in
  auto)
    if [[ -e /dev/kvm ]] && has_cmd kvm-ok 2>/dev/null; then
      accel_args=(-accel kvm)
    elif [[ -e /dev/kvm ]]; then
      accel_args=(-accel kvm)
    else
      accel_args=(-accel tcg)
    fi
    ;;
  kvm) accel_args=(-accel kvm) ;;
  tcg) accel_args=(-accel tcg) ;;
  *) accel_args=() ;;
esac

gdb_args=()
[[ "$GDB" = "1" ]] && gdb_args=(-S -s)

graphics_args=()
if [[ "$GRAPHICS" = "none" ]]; then
  graphics_args=(-nographic)
else
  graphics_args=(-display sdl)
fi

# --- Mode runners ---
run_uefi() {
  ensure_img
  local OVMF_CODE OVMF_VARS
  if ! OVMF_CODE="$(find_ovmf)"; then
    echo "❗ OVMF firmware not found."
    echo "   Install it:   sudo apt-get update && sudo apt-get install -y ovmf"
    exit 1
  fi
  OVMF_VARS="$(find_ovmf_vars || true)"
  local vars_arg=()
  if [[ -n "${OVMF_VARS:-}" ]]; then
    # Copy VARS to a temp file so firmware NVRAM writes don't need root
    local tmpvars
    tmpvars="$(mktemp -t OVMF_VARS.XXXXXX.fd)"
    cp "$OVMF_VARS" "$tmpvars"
    vars_arg=(-drive if=pflash,format=raw,file="$tmpvars")
  fi

  exec qemu-system-x86_64 \
    -machine "$MACHINE" -m "$MEM" -cpu max -smp "$CPUS" \
    -serial stdio "${graphics_args[@]}" "${accel_args[@]}" "${gdb_args[@]}" \
    -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE" \
    "${vars_arg[@]}" \
    -drive file="$IMG",if=none,format=raw,id=esp \
    -device virtio-blk-pci,drive=esp
}

run_bios() {
  echo "⚠ BIOS path not wired yet. Use UEFI with an ESP image instead."
  echo "   (Later, provide a multiboot kernel + BIOS boot sector, or keep UEFI.)"
  exit 2
}

# --- Dispatch ---
case "$MODE" in
  uefi) run_uefi ;;
  bios) run_bios ;;
  *) echo "Unknown MODE: $MODE"; exit 2 ;;
esac

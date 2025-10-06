//! GDT/IDT shell for Asteria OS.
//!
//! Goal:
//! - **devhost (default)**: no-op init so CI/Codespaces builds run immediately.
//! - **OS mode (no_std)**: leave bootloader-provided tables in place for first boot,
//!   but expose `sti/cli/hlt` helpers and a single `init()` call site to wire the
//!   real GDT/IDT later.
//!
//! Later (OS mode): we will create a proper TSS, GDT, and IDT (x86_64), program
//! the timer IRQ, and point each entry to an ISR. That step will add the
//! `x86_64` crate behind `#[cfg(not(feature = "devhost"))]` only.

#![allow(dead_code)]
#![allow(unused_variables)]

/// Initialize CPU tables.
/// - In devhost: does nothing.
/// - In OS mode: currently keeps bootloader tables; safe first mile.
pub fn init() {
    #[cfg(feature = "devhost")]
    {
        // Nothing to do on the host build.
    }

    #[cfg(all(not(feature = "devhost"), any(target_arch = "x86_64", target_arch = "x86")))]
    unsafe {
        // First mile: keep the bootloader's GDT/IDT.
        // Later, replace with explicit GDT/IDT/TSS setup and `lidt/lgdt`.
        // (See TODOs below.)
    }
}

/// Globally enable maskable interrupts (x86/x86_64).
#[inline]
pub fn enable_interrupts() {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    unsafe {
        core::arch::asm!("sti", options(nomem, nostack, preserves_flags));
    }
}

/// Globally disable maskable interrupts (x86/x86_64).
#[inline]
pub fn disable_interrupts() {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    unsafe {
        core::arch::asm!("cli", options(nomem, nostack, preserves_flags));
    }
}

/// Halt the CPU until the next interrupt (saves cycles in idle/panic paths).
#[inline]
pub fn halt() {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    unsafe {
        core::arch::asm!("hlt", options(nomem, nostack, preserves_flags));
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86")))]
    core::hint::spin_loop();
}

/// One-shot: disable interrupts and halt (used by panic/idle loops).
#[inline]
pub fn cli_hlt() -> ! {
    disable_interrupts();
    loop {
        halt();
    }
}

// -----------------------------
// OS-mode TODO (for later)
// -----------------------------
//
// When you flip out of `devhost` and into real no_std OS mode:
// 1) Add a conditional dependency in `kernel/Cargo.toml`:
//
//    [target.'cfg(not(feature = "devhost"))'.dependencies]
//    x86_64 = "0.15"
//
// 2) Replace `init()` with real setup:
//
//    - Create a Task State Segment (TSS) with an IST entry for double-fault.
//    - Build a GDT with: null, kernel_code, kernel_data, TSS.
//    - Load GDT via `lgdt`; reload CS/DS (far jump).
//    - Build an IDT; set handlers for at least: breakpoint, double-fault,
//      page-fault, timer (PIT/HPET), keyboard (if using PS/2).
//    - Load IDT via `lidt`.
//    - In your PIT/HPET ISR, call `crate::time::Clock::tick()`.
//
// Using `x86_64::structures::{gdt::*, idt::*, tss::*}` keeps this code clear
// and avoids manual bit-fiddling of descriptor words.


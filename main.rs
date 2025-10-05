// Asteria OS kernel
// Devhost mode: build as a normal binary so CI and Codespaces can run it.
// OS mode (no_std + no_main) will live behind `#[cfg(not(feature="devhost"))]`.

#[cfg(feature = "devhost")]
fn main() {
    println!("Asteria OS (devhost) — Hexagram HUD scaffold ready. Build OK.");
}

#[cfg(not(feature = "devhost"))]
#![no_std]
#[cfg(not(feature = "devhost"))]
#![no_main]

#[cfg(not(feature = "devhost"))]
mod os_mode {
    use core::panic::PanicInfo;
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! { loop {} }

    // Bootloader entry will be added here later.
}

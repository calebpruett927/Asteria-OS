//! Panic handling for Asteria OS (dual-mode).
//!
//! - **devhost (default)**: install a pretty stderr hook so panics
//!   show file:line:col and a message in Codespaces/CI.
//! - **OS mode (no_std)**: halt the CPU safely; later you can add a
//!   framebuffer/serial panic screen.

#[cfg(feature = "devhost")]
pub fn install_panic_hook() {
    use std::panic;
    use std::time::{SystemTime, UNIX_EPOCH};

    panic::set_hook(Box::new(|info| {
        let ts_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_millis())
            .unwrap_or(0);

        eprintln!("\n=== Asteria OS panic (devhost) ===");
        if let Some(loc) = info.location() {
            eprintln!("at {}:{}:{}", loc.file(), loc.line(), loc.column());
        }

        // Try &str, then String; otherwise show a generic note.
        if let Some(msg) = info.payload().downcast_ref::<&str>() {
            eprintln!("message: {msg}");
        } else if let Some(msg) = info.payload().downcast_ref::<String>() {
            eprintln!("message: {msg}");
        } else {
            eprintln!("message: <non-string payload>");
        }

        eprintln!("epoch_ms: {ts_ms}");
        eprintln!("==================================\n");
    }));
}

// no_std OS-mode panic: disable interrupts (implicitly by HLT loop) and halt.
#[cfg(not(feature = "devhost"))]
use core::panic::PanicInfo;

#[cfg(not(feature = "devhost"))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // OPTIONAL: later, write a minimal message to serial (0x3F8) or draw a panic
    // rectangle on the framebuffer. Keep it simple for first bring-up.

    loop {
        // Halt until next interrupt to avoid burning CPU; keeps system stable at panic site.
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        unsafe {
            core::arch::asm!("hlt");
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64")))]
        core::hint::spin_loop();
    }
}


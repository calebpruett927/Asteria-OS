//! Asteria OS — minimal logger (dual mode).
//!
//! - **devhost (default)**: pretty stderr with level + file:line + timestamp.
//! - **OS mode (no_std)**: writes to serial COM1 (0x3F8) without allocation.
//!
//! Public API:
//!   logger::init();                 // set up sinks (devhost hook / serial init)
//!   logger::set_level(Level::Info); // runtime min level
//!   macros: log_error!, log_warn!, log_info!, log_debug!, log_trace!
//!
//! No dependencies; safe to use very early.

#![allow(dead_code)]
#![allow(unused_variables)]

use core::sync::atomic::{AtomicU8, Ordering::Relaxed};

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Level {
    Error = 0,
    Warn  = 1,
    Info  = 2,
    Debug = 3,
    Trace = 4,
}

static MIN_LEVEL: AtomicU8 = AtomicU8::new(Level::Info as u8);

#[inline]
pub fn set_level(level: Level) {
    MIN_LEVEL.store(level as u8, Relaxed);
}

#[inline]
fn enabled(level: Level) -> bool {
    (level as u8) <= MIN_LEVEL.load(Relaxed)
}

pub fn init() {
    #[cfg(feature = "devhost")]
    devhost_init();

    #[cfg(not(feature = "devhost"))]
    unsafe { serial_init(); }
}

#[cfg(feature = "devhost")]
fn devhost_init() {
    // nothing mandatory; stderr is ready. You can also install the panic hook:
    // crate::panic::install_panic_hook();
}

#[doc(hidden)]
pub fn _log(level: Level, file: &str, line: u32, args: core::fmt::Arguments) {
    if !enabled(level) { return; }

    #[cfg(feature = "devhost")]
    {
        use std::io::Write as _;
        use std::time::{SystemTime, UNIX_EPOCH};
        let ts = SystemTime::now()
            .duration_since(UNIX_EPOCH).map(|d| d.as_millis()).unwrap_or(0);
        let (tag, color) = match level {
            Level::Error => ("ERROR", "\x1b[31m"),
            Level::Warn  => ("WARN ", "\x1b[33m"),
            Level::Info  => ("INFO ", "\x1b[36m"),
            Level::Debug => ("DEBUG", "\x1b[35m"),
            Level::Trace => ("TRACE", "\x1b[90m"),
        };
        let reset = "\x1b[0m";
        let mut stderr = std::io::stderr();
        let _ = write!(stderr, "{color}[{tag}] {ts} {file}:{line}: ", color=color, tag=tag, ts=ts, file=file, line=line);
        let _ = stderr.write_fmt(args);
        let _ = writeln!(stderr, "{reset}");
    }

    #[cfg(not(feature = "devhost"))]
    {
        // Prefix: [L] file:line
        serial_write_b(b'[');
        serial_write_str(match level {
            Level::Error => "E",
            Level::Warn  => "W",
            Level::Info  => "I",
            Level::Debug => "D",
            Level::Trace => "T",
        });
        serial_write_b(b']'); serial_write_b(b' ');
        serial_write_str(file); serial_write_b(b':');
        decimal_u32(line);
        serial_write_b(b' ');

        // Body
        let mut w = SerialWriter;
        let _ = core::fmt::Write::write_fmt(&mut w, args);
        serial_write_b(b'\r'); serial_write_b(b'\n');
    }
}

// ----- Macros ---------------------------------------------------------------

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        $crate::kernel::logger::_log($crate::kernel::logger::Level::Error, file!(), line!(), format_args!($($arg)*))
    }
}
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        $crate::kernel::logger::_log($crate::kernel::logger::Level::Warn, file!(), line!(), format_args!($($arg)*))
    }
}
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        $crate::kernel::logger::_log($crate::kernel::logger::Level::Info, file!(), line!(), format_args!($($arg)*))
    }
}
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        $crate::kernel::logger::_log($crate::kernel::logger::Level::Debug, file!(), line!(), format_args!($($arg)*))
    }
}
#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)*) => {
        $crate::kernel::logger::_log($crate::kernel::logger::Level::Trace, file!(), line!(), format_args!($($arg)*))
    }
}

// ----- OS-mode serial sink (x86/x86_64) ------------------------------------

#[cfg(not(feature = "devhost"))]
struct SerialWriter;

#[cfg(not(feature = "devhost"))]
impl core::fmt::Write for SerialWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        serial_write_str(s);
        Ok(())
    }
}

#[cfg(not(feature = "devhost"))]
#[inline(always)]
fn serial_write_b(b: u8) {
    // Wait for THR empty
    while (unsafe { inb(COM1 + 5) } & 0x20) == 0 {}
    unsafe { outb(COM1, b) };
}

#[cfg(not(feature = "devhost"))]
fn serial_write_str(s: &str) {
    for &b in s.as_bytes() {
        // Map '\n' to CRLF for common terminals.
        if b == b'\n' { serial_write_b(b'\r'); }
        serial_write_b(b);
    }
}

#[cfg(not(feature = "devhost"))]
const COM1: u16 = 0x3F8;

#[cfg(not(feature = "devhost"))]
unsafe fn serial_init() {
    // 115200 8N1, FIFO on. IRQs off (polled TX).
    outb(COM1 + 1, 0x00);       // disable interrupts
    outb(COM1 + 3, 0x80);       // enable DLAB
    outb(COM1 + 0, 0x01);       // divisor low (1) => 115200
    outb(COM1 + 1, 0x00);       // divisor high
    outb(COM1 + 3, 0x03);       // 8 bits, no parity, one stop
    outb(COM1 + 2, 0xC7);       // enable FIFO, clear, 14-byte threshold
    outb(COM1 + 4, 0x0B);       // DTR, RTS, OUT2
}

#[cfg(not(feature = "devhost"))]
#[inline(always)]
unsafe fn outb(port: u16, val: u8) {
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    core::arch::asm!("out dx, al", in("dx") port, in("al") val, options(nomem, nostack, preserves_flags));
}

#[cfg(not(feature = "devhost"))]
#[inline(always)]
unsafe fn inb(port: u16) -> u8 {
    let mut v: u8;
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    unsafe { core::arch::asm!("in al, dx", in("dx") port, out("al") v, options(nomem, nostack, preserves_flags)); }
    v
}

// Minimal decimal writer (no alloc) for line numbers.
#[cfg(not(feature = "devhost"))]
fn decimal_u32(mut n: u32) {
    let mut buf = [0u8; 10];
    let mut i = 10;
    if n == 0 { serial_write_b(b'0'); return; }
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    for &b in &buf[i..] { serial_write_b(b); }
}

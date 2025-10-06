//! Asteria UI — Painter
//!
//! Two build faces behind the `devhost` feature flag:
//! - **devhost (default):** in-memory RGBA surface (Vec<u32>) so Codespaces/CI can run.
//! - **OS mode (no_std):** zero-alloc stub that keeps the API stable until HAL framebuffer lands.
//!
//! The `Painter` implements `crate::widgets::PixelSink`, so you can call
//! `panel/progress/button` today. In devhost, they actually draw into RAM;
//! in OS mode they are no-ops until you wire the real framebuffer.

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::widgets::{Color, PixelSink};

/// High-level drawing façade.
pub struct Painter {
    w: i32,
    h: i32,

    #[cfg(feature = "devhost")]
    buf: Vec<u32>, // RGBA8888

    frame_count: u64,
}

impl Painter {
    // ---------- Constructors ----------

    /// Create a devhost painter with an RGBA buffer (e.g., 640x360).
    #[cfg(feature = "devhost")]
    pub fn new_devhost(w: i32, h: i32) -> Self {
        let w = w.max(1);
        let h = h.max(1);
        let buf = vec![0u32; (w as usize) * (h as usize)];
        Self { w, h, buf, frame_count: 0 }
    }

    /// Create an OS-mode painter with a placeholder surface.
    /// Replace with a framebuffer-backed constructor later.
    #[cfg(not(feature = "devhost"))]
    pub fn new_os_stub(w: i32, h: i32) -> Self {
        let w = w.max(1);
        let h = h.max(1);
        Self { w, h, frame_count: 0 }
    }

    // ---------- Surface info ----------

    #[inline] pub fn width(&self) -> i32 { self.w }
    #[inline] pub fn height(&self) -> i32 { self.h }
    #[inline] pub fn frame(&self) -> u64 { self.frame_count }

    // ---------- Basic drawing ----------

    /// Clear the surface to a solid color.
    pub fn clear_rgb(&mut self, r: u8, g: u8, b: u8) {
        #[cfg(feature = "devhost")]
        {
            let px = ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xFF;
            self.buf.fill(px);
        }
        #[cfg(not(feature = "devhost"))]
        {
            // no-op until HAL is wired
            let _ = (r, g, b);
        }
    }

    /// Put a single pixel at (x,y). Out-of-bounds safely ignored.
    pub fn put_px(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        if x < 0 || y < 0 || x >= self.w || y >= self.h { return; }
        #[cfg(feature = "devhost")]
        {
            let idx = (y as usize) * (self.w as usize) + (x as usize);
            let px = ((r as u32) << 24) | ((g as u32) << 16) | ((b as u32) << 8) | 0xFF;
            // Safety: idx bounds checked above.
            unsafe { *self.buf.get_unchecked_mut(idx) = px; }
        }
        #[cfg(not(feature = "devhost"))]
        {
            let _ = (x, y, r, g, b); // no-op until HAL is wired
        }
    }

    /// Present the frame (no-op for now). Call each loop to bump `frame_count`.
    pub fn present(&mut self) {
        self.frame_count = self.frame_count.wrapping_add(1);

        // Optional: dump a PPM for quick visual inspection (devhost only).
        // Enable by setting env ASTERIA_DUMP_PPM=path.ppm and calling once.
        #[cfg(feature = "devhost")]
        {
            if let Ok(path) = std::env::var("ASTERIA_DUMP_PPM") {
                let _ = self.dump_ppm(&path);
            }
        }
    }

    // ---------- Devhost helpers ----------

    /// DEVHOST: write current buffer to a PPM file (simple debug path).
    #[cfg(feature = "devhost")]
    pub fn dump_ppm(&self, path: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::{BufWriter, Write};
        let mut f = BufWriter::new(File::create(path)?);
        writeln!(f, "P6\n{} {}\n255", self.w, self.h)?;
        // Convert RGBA8888 to PPM RGB
        for p in &self.buf {
            let r = (p >> 24) as u8;
            let g = (p >> 16) as u8;
            let b = (p >> 8) as u8;
            f.write_all(&[r, g, b])?;
        }
        Ok(())
    }
}

// ---------- PixelSink impl so widgets can draw ----------

impl PixelSink for Painter {
    #[inline]
    fn size(&self) -> (i32, i32) { (self.w, self.h) }

    #[inline]
    fn put(&mut self, x: i32, y: i32, c: Color) {
        self.put_px(x, y, c.r, c.g, c.b);
    }
}

// ---------- Convenience retained-mode-ish helpers ----------

impl Painter {
    /// Fill a rectangle.
    pub fn rect(&mut self, x: i32, y: i32, w: i32, h: i32, r: u8, g: u8, b: u8) {
        let (sw, sh) = (self.w, self.h);
        let x0 = x.max(0);
        let y0 = y.max(0);
        let x1 = (x + w).min(sw);
        let y1 = (y + h).min(sh);
        if x1 <= x0 || y1 <= y0 { return; }
        for yy in y0..y1 {
            for xx in x0..x1 {
                self.put_px(xx, yy, r, g, b);
            }
        }
    }
}


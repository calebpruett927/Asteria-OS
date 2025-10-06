//! Minimal immediate-mode widgets for Asteria UI.
//! No allocation, `no_std` friendly, and independent of any concrete framebuffer.
//!
//! You can integrate this with your renderer by implementing `PixelSink` for it.
//! Example later: impl PixelSink for your `Painter` with a `put_px(x,y,Color)` method.

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(clippy::needless_return)]

#[cfg(not(feature = "devhost"))]
extern crate core as std; // allow `no_std` builds to use core types via `std::` path here

// ---------- Core types ----------

/// RGBA color (alpha currently ignored by default fills).
#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self { Self { r, g, b, a: 255 } }
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { Self { r, g, b, a } }

    // Some handy palette entries
    pub const BG: Self = Self::rgb(16, 16, 24);
    pub const PANEL: Self = Self::rgb(32, 32, 48);
    pub const PANEL_HI: Self = Self::rgb(48, 48, 72);
    pub const FG: Self = Self::rgb(220, 220, 230);
    pub const ACCENT: Self = Self::rgb(80, 160, 255);
    pub const WARN: Self = Self::rgb(255, 180, 0);
    pub const ERR: Self = Self::rgb(220, 80, 80);
}

/// Rectangle in integer pixel coords.
#[derive(Copy, Clone)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}
impl Rect {
    pub const fn new(x: i32, y: i32, w: i32, h: i32) -> Self { Self { x, y, w, h } }
    #[inline] pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && py >= self.y && px < self.x + self.w && py < self.y + self.h
    }
}

/// Minimal input snapshot for immediate-mode interactions.
#[derive(Copy, Clone, Default)]
pub struct UiState {
    pub mouse_x: i32,
    pub mouse_y: i32,
    pub pressed: bool,     // true if primary button currently down
    pub just_pressed: bool, // optional edge; if you don't track it, set false
    pub just_released: bool,
}

// ---------- Pixel sink abstraction ----------

/// Anything that can set pixels and reports its surface size.
pub trait PixelSink {
    fn size(&self) -> (i32, i32);
    fn put(&mut self, x: i32, y: i32, c: Color);

    /// Fill a rect with a solid color (clipped to surface).
    fn fill_rect(&mut self, r: Rect, c: Color) {
        let (sw, sh) = self.size();
        let x0 = r.x.max(0);
        let y0 = r.y.max(0);
        let x1 = (r.x + r.w).min(sw);
        let y1 = (r.y + r.h).min(sh);
        if x1 <= x0 || y1 <= y0 { return; }
        for y in y0..y1 {
            for x in x0..x1 {
                self.put(x, y, c);
            }
        }
    }

    /// 1-px border (outside edges clipped to surface).
    fn stroke_rect(&mut self, r: Rect, c: Color) {
        let (sw, sh) = self.size();
        for x in r.x.max(0)..(r.x + r.w).min(sw) {
            self.put(x, r.y.max(0), c);
            self.put(x, (r.y + r.h - 1).min(sh - 1), c);
        }
        for y in r.y.max(0)..(r.y + r.h).min(sh) {
            self.put(r.x.max(0), y, c);
            self.put((r.x + r.w - 1).min(sw - 1), y, c);
        }
    }
}

// ---------- Widgets ----------

/// Panel with a subtle header band. Returns the inner content area.
pub fn panel<S: PixelSink>(sink: &mut S, rect: Rect) -> Rect {
    sink.fill_rect(rect, Color::PANEL);
    // header band
    let header = Rect::new(rect.x, rect.y, rect.w, 18.max(1));
    sink.fill_rect(header, Color::PANEL_HI);
    sink.stroke_rect(rect, Color::rgb(24, 24, 36));
    // content area (padding 8 px)
    Rect::new(rect.x + 8, rect.y + 22, (rect.w - 16).max(0), (rect.h - 30).max(0))
}

/// Very simple label: draws a 1-px baseline so you can see placement even before fonts land.
/// Once you have a bitmap font, swap this to real text glyphs.
pub fn label_baseline<S: PixelSink>(sink: &mut S, x: i32, y: i32, _text: &str) {
    // Baseline line: short accent dash
    for dx in 0..24 {
        sink.put(x + dx, y, Color::FG);
    }
}

/// Progress bar in [0,1], clamped. Returns the filled pixel width.
pub fn progress<S: PixelSink>(sink: &mut S, rect: Rect, t: f32) -> i32 {
    let t = if t.is_nan() { 0.0 } else { t.clamp(0.0, 1.0) };
    sink.fill_rect(rect, Color::rgb(40, 40, 56));
    let fill_w = ((rect.w as f32) * t).round() as i32;
    if fill_w > 0 {
        sink.fill_rect(Rect::new(rect.x, rect.y, fill_w.min(rect.w), rect.h), Color::ACCENT);
    }
    sink.stroke_rect(rect, Color::rgb(28, 28, 40));
    fill_w
}

/// Button (hit-test only for now). Returns `true` if a click was detected this frame.
/// Use `UiState.just_released` for edge-triggered clicks if you have it; else fallback to `pressed`.
pub fn button<S: PixelSink>(sink: &mut S, rect: Rect, _label: &str, ui: UiState) -> bool {
    let hovered = rect.contains(ui.mouse_x, ui.mouse_y);
    let base = if hovered { Color::rgb(52, 52, 72) } else { Color::rgb(44, 44, 64) };
    sink.fill_rect(rect, base);
    sink.stroke_rect(rect, if hovered { Color::ACCENT } else { Color::rgb(28, 28, 40) });
    // Later: draw label with your font in the center.
    let clicked = if ui.just_released { hovered } else { hovered && ui.pressed };
    return clicked;
}

// ---------- Integrating with your renderer ----------
//
// Implement `PixelSink` for your Painter once `paint.rs` exposes a `put_px` and `size()`:
//
// impl PixelSink for Painter<'_> {
//     fn size(&self) -> (i32, i32) { (self.width() as i32, self.height() as i32) }
//     fn put(&mut self, x: i32, y: i32, c: Color) {
//         self.put_px(x as usize, y as usize, c.r, c.g, c.b);
//     }
// }
//
// Then you can do:
//
// let content = panel(&mut painter, Rect::new(12, 12, 460, 120));
// label_baseline(&mut painter, content.x + 8, content.y + 12, "Hexagram HUD — AsteriaOS");
// let _ = progress(&mut painter, Rect::new(content.x, content.y + 24, content.w, 10), 0.42);
// let clicked = button(&mut painter, Rect::new(content.x, content.y + 40, 96, 20), "OK", ui);


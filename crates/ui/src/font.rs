//! Tiny 5x7 bitmap font for Asteria UI (no_std-ready, no allocations).
//!
//! Exposes:
//!   - FONT_W, FONT_H
//!   - draw_text(sink, x, y, text, fg, bg)
//!   - draw_glyph(sink, x, y, ch, fg, bg)
//!   - measure_text(text) -> (w, h)
//!
//! Integrates with widgets via the PixelSink trait. Works in devhost and OS mode.
//!
//! Notes:
//! - Glyph cell is 6x8: a 5x7 glyph plus 1px spacing right and 1px spacing below.
//! - Lowercase is rendered as uppercase.
//! - Unknown characters draw as a 5x7 outline box.

#![allow(dead_code)]
#![allow(unused_variables)]

use crate::widgets::{Color, PixelSink};

pub const FONT_W: i32 = 6; // advance (5 px glyph + 1 px spacing)
pub const FONT_H: i32 = 8; // cell height (7 px glyph + 1 px spacing)

/// Render a UTF-8 string (ASCII subset supported) at (x, y).
/// If `bg` is Some, fills each glyph cell with that color before drawing.
pub fn draw_text<S: PixelSink>(sink: &mut S, mut x: i32, y: i32, text: &str, fg: Color, bg: Option<Color>) -> (i32, i32) {
    for ch in text.chars() {
        let w = draw_glyph(sink, x, y, ch, fg, bg);
        x += w;
    }
    let (w, h) = measure_text(text);
    (w, h)
}

/// Draw a single glyph. Returns advance in pixels (FONT_W).
pub fn draw_glyph<S: PixelSink>(sink: &mut S, x: i32, y: i32, ch: char, fg: Color, bg: Option<Color>) -> i32 {
    if let Some(bg) = bg {
        // Fill the whole cell first (optional background)
        fill_cell(sink, x, y, bg);
    }

    let glyph = glyph5x7(ch).unwrap_or(&GLYPH_BOX);
    // Draw 5x7 pixels, MSB..LSB across 5 columns.
    for (row, bits) in glyph.iter().enumerate() {
        let yy = y + row as i32;
        let mut mask = 0b1_0000u8; // leftmost of 5 bits
        for col in 0..5 {
            if (bits & mask) != 0 {
                let xx = x + col as i32;
                sink.put(xx, yy, fg);
            }
            mask >>= 1;
        }
    }
    FONT_W
}

/// Compute the pixel size of `text` using fixed advance.
pub fn measure_text(text: &str) -> (i32, i32) {
    let len = text.chars().count() as i32;
    (len * FONT_W, FONT_H)
}

/// Fill one glyph cell (6x8) at (x,y).
fn fill_cell<S: PixelSink>(sink: &mut S, x: i32, y: i32, c: Color) {
    let (sw, sh) = sink.size();
    let x0 = x.max(0);
    let y0 = y.max(0);
    let x1 = (x + FONT_W).min(sw);
    let y1 = (y + FONT_H).min(sh);
    if x1 <= x0 || y1 <= y0 { return; }
    for yy in y0..y1 {
        for xx in x0..x1 {
            sink.put(xx, yy, c);
        }
    }
}

// ---------- 5x7 glyph ROM ----------
// Each glyph is 7 rows of 5 bits (MSB=left pixel). Only ASCII subset is provided.

const GLYPH_SP: [u8; 7] = [0,0,0,0,0,0,0];

const GLYPH_BOX: [u8; 7] = [
    0b11111,
    0b10001,
    0b10001,
    0b10001,
    0b10001,
    0b10001,
    0b11111,
];

// Digits 0–9
const D0: [u8;7] = [0b01110,0b10001,0b10011,0b10101,0b11001,0b10001,0b01110];
const D1: [u8;7] = [0b00100,0b01100,0b00100,0b00100,0b00100,0b00100,0b01110];
const D2: [u8;7] = [0b01110,0b10001,0b00001,0b00010,0b00100,0b01000,0b11111];
const D3: [u8;7] = [0b11110,0b00001,0b00001,0b01110,0b00001,0b00001,0b11110];
const D4: [u8;7] = [0b00010,0b00110,0b01010,0b10010,0b11111,0b00010,0b00010];
const D5: [u8;7] = [0b11111,0b10000,0b11110,0b00001,0b00001,0b10001,0b01110];
const D6: [u8;7] = [0b00110,0b01000,0b10000,0b11110,0b10001,0b10001,0b01110];
const D7: [u8;7] = [0b11111,0b00001,0b00010,0b00100,0b01000,0b01000,0b01000];
const D8: [u8;7] = [0b01110,0b10001,0b10001,0b01110,0b10001,0b10001,0b01110];
const D9: [u8;7] = [0b01110,0b10001,0b10001,0b01111,0b00001,0b00010,0b01100];

// Uppercase A–Z
const A_: [u8;7] = [0b01110,0b10001,0b10001,0b11111,0b10001,0b10001,0b10001];
const B_: [u8;7] = [0b11110,0b10001,0b10001,0b11110,0b10001,0b10001,0b11110];
const C_: [u8;7] = [0b01110,0b10001,0b10000,0b10000,0b10000,0b10001,0b01110];
const D_: [u8;7] = [0b11100,0b10010,0b10001,0b10001,0b10001,0b10010,0b11100];
const E_: [u8;7] = [0b11111,0b10000,0b10000,0b11110,0b10000,0b10000,0b11111];
const F_: [u8;7] = [0b11111,0b10000,0b10000,0b11110,0b10000,0b10000,0b10000];
const G_: [u8;7] = [0b01110,0b10001,0b10000,0b10111,0b10001,0b10001,0b01110];
const H_: [u8;7] = [0b10001,0b10001,0b10001,0b11111,0b10001,0b10001,0b10001];
const I_: [u8;7] = [0b01110,0b00100,0b00100,0b00100,0b00100,0b00100,0b01110];
const J_: [u8;7] = [0b00001,0b00001,0b00001,0b00001,0b10001,0b10001,0b01110];
const K_: [u8;7] = [0b10001,0b10010,0b10100,0b11000,0b10100,0b10010,0b10001];
const L_: [u8;7] = [0b10000,0b10000,0b10000,0b10000,0b10000,0b10000,0b11111];
const M_: [u8;7] = [0b10001,0b11011,0b10101,0b10101,0b10001,0b10001,0b10001];
const N_: [u8;7] = [0b10001,0b11001,0b10101,0b10011,0b10001,0b10001,0b10001];
const O_: [u8;7] = [0b01110,0b10001,0b10001,0b10001,0b10001,0b10001,0b01110];
const P_: [u8;7] = [0b11110,0b10001,0b10001,0b11110,0b10000,0b10000,0b10000];
const Q_: [u8;7] = [0b01110,0b10001,0b10001,0b10001,0b10101,0b10010,0b01101];
const R_: [u8;7] = [0b11110,0b10001,0b10001,0b11110,0b10100,0b10010,0b10001];
const S_: [u8;7] = [0b01111,0b10000,0b10000,0b01110,0b00001,0b00001,0b11110];
const T_: [u8;7] = [0b11111,0b00100,0b00100,0b00100,0b00100,0b00100,0b00100];
const U_: [u8;7] = [0b10001,0b10001,0b10001,0b10001,0b10001,0b10001,0b01110];
const V_: [u8;7] = [0b10001,0b10001,0b10001,0b10001,0b01010,0b01010,0b00100];
const W_: [u8;7] = [0b10001,0b10001,0b10001,0b10101,0b10101,0b11011,0b10001];
const X_: [u8;7] = [0b10001,0b01010,0b00100,0b00100,0b00100,0b01010,0b10001];
const Y_: [u8;7] = [0b10001,0b01010,0b00100,0b00100,0b00100,0b00100,0b00100];
const Z_: [u8;7] = [0b11111,0b00001,0b00010,0b00100,0b01000,0b10000,0b11111];

// Punctuation commonly needed by HUD
const COLON: [u8;7] = [0b00000,0b00100,0b00100,0b00000,0b00100,0b00100,0b00000];
const DOT:   [u8;7] = [0b00000,0b00000,0b00000,0b00000,0b00000,0b00100,0b00100];
const DASH:  [u8;7] = [0b00000,0b00000,0b00000,0b01110,0b00000,0b00000,0b00000];
const SLASH: [u8;7] = [0b00001,0b00010,0b00100,0b01000,0b10000,0b00000,0b00000];
const EQ:    [u8;7] = [0b00000,0b01110,0b00000,0b01110,0b00000,0b00000,0b00000];
const LPAR:  [u8;7] = [0b00010,0b00100,0b01000,0b01000,0b01000,0b00100,0b00010];
const RPAR:  [u8;7] = [0b01000,0b00100,0b00010,0b00010,0b00010,0b00100,0b01000];
const COMMA: [u8;7] = [0b00000,0b00000,0b00000,0b00000,0b00100,0b00100,0b01000];
const PCT:   [u8;7] = [0b11000,0b11001,0b00010,0b00100,0b01000,0b10011,0b00011];
const USCR:  [u8;7] = [0b00000,0b00000,0b00000,0b00000,0b00000,0b00000,0b11111];
const SPACE: [u8;7] = GLYPH_SP;

/// Map ASCII char to glyph (5x7). Lowercase maps to uppercase.
fn glyph5x7(ch: char) -> Option<&'static [u8;7]> {
    let c = ch;
    match c {
        ' ' => Some(&SPACE),
        '0' => Some(&D0), '1' => Some(&D1), '2' => Some(&D2), '3' => Some(&D3), '4' => Some(&D4),
        '5' => Some(&D5), '6' => Some(&D6), '7' => Some(&D7), '8' => Some(&D8), '9' => Some(&D9),
        'A' => Some(&A_), 'B' => Some(&B_), 'C' => Some(&C_), 'D' => Some(&D_), 'E' => Some(&E_),
        'F' => Some(&F_), 'G' => Some(&G_), 'H' => Some(&H_), 'I' => Some(&I_), 'J' => Some(&J_),
        'K' => Some(&K_), 'L' => Some(&L_), 'M' => Some(&M_), 'N' => Some(&N_), 'O' => Some(&O_),
        'P' => Some(&P_), 'Q' => Some(&Q_), 'R' => Some(&R_), 'S' => Some(&S_), 'T' => Some(&T_),
        'U' => Some(&U_), 'V' => Some(&V_), 'W' => Some(&W_), 'X' => Some(&X_), 'Y' => Some(&Y_),
        'Z' => Some(&Z_),
        ':' => Some(&COLON), '.' => Some(&DOT), '-' => Some(&DASH), '/' => Some(&SLASH),
        '=' => Some(&EQ), '(' => Some(&LPAR), ')' => Some(&RPAR), ',' => Some(&COMMA),
        '%' => Some(&PCT), '_' => Some(&USCR),
        // lowercase → uppercase
        'a'..='z' => glyph5x7(c.to_ascii_uppercase()),
        _ => None,
    }
}

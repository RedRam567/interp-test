pub mod movement;
pub mod player;
pub mod state;
pub mod time;

use macroquad::prelude::*;

const DBG_OPACITY: f32 = 0.25;
const DBG_OPACITY2: f32 = 0.5;
pub const DBG_PREV: Color = Color { a: DBG_OPACITY, ..RED };
pub const DBG_NOW: Color = Color { a: DBG_OPACITY, ..GREEN };
pub const DBG_INTERP: Color = Color { a: DBG_OPACITY2, ..BLUE };

pub fn dbg_circle(pos: Vec2) {
    draw_circle(pos.x, pos.y, 5.0, WHITE);
}

pub fn dbg_arrow(pos: Vec2, dir_len: Vec2, color: Color) {
    const THICKNESS: f32 = 2.0;
    let end = pos + dir_len;
    draw_line(pos.x, pos.y, end.x, end.y, THICKNESS, color)
}

pub fn dbg_line(pos: Vec2, end: Vec2, color: Color) {
    const THICKNESS: f32 = 2.0;
    draw_line(pos.x, pos.y, end.x, end.y, THICKNESS, color)
}

fn lerp_fast2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2::new(lerp_fast(a.x, b.x, t), lerp_fast(a.y, b.y, t))
}

fn lerp_precise2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    Vec2::new(lerp_precise(a.x, b.x, t), lerp_precise(a.y, b.y, t))
}

// NOTE: personally: lerp_precise is better in most cases, but lerp_fast good for similar values
/// Linear interpolation. About `a` when `t` is 0.0, about `b` when `t` is 1.0. Monotinic.
/// Not guarenteed to be exact. Extrapolates when outside of 0..=1.
/// Precision issues when `a` and `b` have very different exponents.
///
/// See <https://en.wikipedia.org/wiki/Linear_interpolation#Programming_language_support>
fn lerp_fast(a: f32, b: f32, t: f32) -> f32 {
    // same as: a + t * (b - a)
    t.mul_add(b - a, a)
}

/// Linear interpolation. Exactly `a` when `t` is 0.0, exactly `b` when `t` is 1.0. Not monotonic.
/// Extrapolates when outside of 0..=1. Probably as best as you can get, simply
/// scaling `b` by `t`, and `a` by inverse `t`, then adding.
///
/// See <https://en.wikipedia.org/wiki/Linear_interpolation#Programming_language_support>
fn lerp_precise(a: f32, b: f32, t: f32) -> f32 {
    // same as: (1.0 - t) * a + t * b
    a.mul_add(1.0 - t, t * b)
}

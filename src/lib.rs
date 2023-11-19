pub mod movement;
pub mod player;
pub mod spline;
pub mod state;
pub mod time;

// pub mod dbg;

use macroquad::prelude::*;

const DBG_OPACITY: f32 = 0.25;
const DBG_OPACITY2: f32 = 0.5;
// const DBG_OPACITY: f32 = 0.0;
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

/// See <https://math.stackexchange.com/questions/13261/how-to-get-a-reflection-vector>
pub fn vec2_mirror(ray: Vec2, mirror: Vec2, start: Vec2) -> Vec2 {
    // ray - 2.0 * (ray.dot(mirror)) * mirror
    // 2.0 * (ray.dot(mirror)) * mirror - ray
    // 2.0 *(mirror.dot(ray))*mirror - ray
    // 2.0 * ()
    // mirror.project_onto(ray)
    // mirror.project_onto(ray)
    // (ray.dot(mirror))*mirror
    let angle = ray.angle_between(mirror);
    rotate_around_point(ray, start, angle)
}

fn rotate_around_point(ray: Vec2, point: Vec2, angle: f32) -> Vec2 {
    let norm = ray - point;
    let x = norm.x * angle.cos() - norm.y * angle.sin();
    let y = norm.x * angle.sin() + norm.y * angle.cos();
    Vec2::new(x, y) + point
}

pub fn rotate_around_origin(vec2: Vec2, angle: f32) -> Vec2 {
    Vec2::new(vec2.y * f32::cos(angle), vec2.x * f32::sin(angle))
}

fn lerp_fast2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    // Vec2 {
    //     x: lerp_fast(a.x, b.x, t),
    //     y: lerp_fast(a.y, b.y, t),
    // };
    Vec2::new(lerp_fast(a.x, b.x, t), lerp_fast(a.y, b.y, t))
}

fn lerp_precise_2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    // Vec2 {
    //     x: lerp_fast(a.x, b.x, t),
    //     y: lerp_fast(a.y, b.y, t),
    // };
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

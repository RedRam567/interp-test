pub mod movement;
pub mod spline;
pub mod time;
pub mod state;

// pub mod dbg;

use macroquad::prelude::*;

use crate::movement::Movement;

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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Player {
    pub movement: Movement,
}

impl Player {
    const PLAYER_SIZE: f32 = 10.0;
    // const PLAYER_ACCEL: f32 = 1.0;
    const PLAYER_COLOR: Color = BLACK;
    // const PLAYER_MAX_SPEED: f32 = 20.0 * SPEED_FACTOR * 1.0;
    // const PLAYER_ACCEL: f32 = 5.0 * SPEED_FACTOR * SPEED_FACTOR;
    // const MAX_SPEED: f32 = 20.0;
    const MAX_SPEED: f32 = 15.0;
    const ACCEL: f32 = 5.0;
    const BASE_FRICTION: f32 = 0.5;
    const SCALING_FRICTION: f32 = 5e-2;
    // const BASE_FRICTION: f32 = 0.0;
    // const SCALING_FRICTION: f32 = 0.0;

    pub fn max_speed(speed_factor: f32) -> f32 {
        Self::MAX_SPEED * speed_factor
    }

    pub fn accel(speed_factor: f32) -> f32 {
        Self::ACCEL * (speed_factor * speed_factor)
    }

    pub fn base_friction(speed_factor: f32) -> f32 {
        Self::BASE_FRICTION * (speed_factor * speed_factor)
    }

    pub fn scaling_friction(speed_factor: f32) -> f32 {
        Self::SCALING_FRICTION * (speed_factor)
    }

    pub fn handle_movement(&mut self, desired_dir: &[Vec2], accel: f32) -> &mut Self {
        let dir = average_input(desired_dir);
        self.movement.accel = dir * accel;
        self
    }

    pub fn desired_dir() -> Vec2 {
        let mut dir = Vec2::ZERO;
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dir.y += -1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dir.y += 1.0;
        }
        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dir.x += -1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dir.x += 1.0;
        }
        dir.normalize_or_zero()
    }

    pub fn draw(&self, prev: &Self, t: f32) -> &Self {
        // let Vec2 { x, y } = self.movement.pos;
        // let Vec2 { x, y } = lerp_fast2(prev.movement.pos, self.movement.pos, t);
        // let Vec2 { x, y } = lerp_fast2(prev.movement.pos, self.movement.pos, t);
        // let Vec2 {x,y} = prev.movement.interp0(&self.movement, t);
        // let Vec2 {x,y} = prev.movement.interp1(&self.movement, t);
        // let Vec2 {x,y} = prev.movement.interp2(&self.movement, t);
        // let Vec2 { x, y } = prev.movement.interp3(&self.movement, t);
        let Vec2 { x, y } = prev.movement.interp(&self.movement, t);
        draw_circle(x, y, Self::PLAYER_SIZE, Self::PLAYER_COLOR);
        self
    }

    pub fn draw_dbg(&self, prev: &Self, _t: f32) -> &Self {
        // let Vec2 { x, y } = self.movement.pos;
        // let Vec2 { x, y } = lerp_fast2(prev.movement.pos, self.movement.pos, t);
        // let Vec2 { x, y } = lerp_fast2(prev.movement.pos, self.movement.pos, t);
        let Vec2 { x, y } = prev.movement.interp0(&self.movement, 1.0);
        // let Vec2 {x,y} = prev.movement.interp1(&self.movement, t);
        // let Vec2 {x,y} = prev.movement.interp2(&self.movement, t);
        // let Vec2 {x,y} = prev.movement.interp3(&self.movement, 1.0);
        draw_circle(x, y, Self::PLAYER_SIZE, DBG_NOW);
        self
    }

    pub fn draw_dbg_prev(&self, prev: &Self, _t: f32) -> &Self {
        // let Vec2 { x, y } = self.movement.pos;
        // let Vec2 { x, y } = lerp_fast2(prev.movement.pos, self.movement.pos, t);
        // let Vec2 { x, y } = lerp_fast2(prev.movement.pos, self.movement.pos, t);
        let Vec2 { x, y } = prev.movement.interp0(&self.movement, 0.0);
        // let Vec2 {x,y} = prev.movement.interp1(&self.movement, t);
        // let Vec2 {x,y} = prev.movement.interp2(&self.movement, t);
        // let Vec2 {x,y} = prev.movement.interp3(&self.movement, 0.0);
        draw_circle(x, y, Self::PLAYER_SIZE, DBG_PREV);
        self
    }
}

// mean. TODO: test bias latest input
// TODO: raw mean vs normalized vs mixed
// #[allow(clippy::let_and_return)]
pub fn average_input(input: &[Vec2]) -> Vec2 {
    let mean = input.iter().sum::<Vec2>() / input.len() as f32;

    // let norm = mean.normalize_or_zero();
    // let ret = lerp_fast2(norm, mean, 0.5);

    let ret = mean;
    // norm

    ret
}

fn lerp_fast2(a: Vec2, b: Vec2, t: f32) -> Vec2 {
    // Vec2 {
    //     x: lerp_fast(a.x, b.x, t),
    //     y: lerp_fast(a.y, b.y, t),
    // };
    Vec2::new(lerp_fast(a.x, b.x, t), lerp_fast(a.y, b.y, t))
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

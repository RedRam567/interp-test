use std::fmt::Display;

use macroquad::prelude::*;

use crate::movement::Movement;
use crate::{lerp_precise_2, DBG_NOW, DBG_PREV};

// TODO: list benefits
// TODO: mean, ignore zero
/// How to average the player input.
/// # Philosophy
/// You want to collect player input as often as possible to reduce latency,
/// but also want want to run game logic at a fixed tps (for numerous benifits).
/// So you have to average the player input during the last tick and use that
/// for your desired direction.
/// In reality you would probably just use mean or mean normalized and call it a
/// day. This enum is for runtime experimenting of it.
/// # Notes
/// `MeanIgnoreZero` and `MeanNormalized` reduces the need for null cancelling movement
/// by about 50% or smth but still allows for null movement (is that ever even useful?)
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AveragingStrategy {
    /// Worst latency, can miss inputs.
    Oldest,
    /// Best latency, can miss inputs.
    Newest,
    /// Mean average, fine grain control, potentially high latency.
    Mean,
    /// Mean average with zero inputs ignored. I would Recomeneded for most scenarios.
    /// Low latency for starting, slightly less fine grain control.
    /// NOTE: if implementing on your own, it'd be better to ignore zeros when
    /// reading input, rather than when updating next tick.
    #[default]
    MeanIgnoreZero,
    /// Mean normalized to 1.0. Possibly better than MeanIgnoreZero.
    /// Lower latency while moving, but more "jittery", less fine grain control.
    MeanNormalized,
    // Mean lerped with MeanNormalized. 0 -> mean, 1.0 -> Normalized
    MeanNormalizedPercent(f32),

    // hmm very interesting middle ground between Mean and MeanIgnoreZero/Normalized
    // MeanIgnoreFirstXZeros(usize),
}

impl AveragingStrategy {
    // expect about 33 items to average for 1000hz and 10 tps
    pub fn average(&self, dirs: &[Vec2]) -> Vec2 {
        match *self {
            AveragingStrategy::Oldest => Self::oldest(dirs),
            AveragingStrategy::Newest => Self::newest(dirs),
            AveragingStrategy::Mean => Self::mean(dirs),
            AveragingStrategy::MeanIgnoreZero => Self::mean_ignore_zero(dirs),
            AveragingStrategy::MeanNormalized => Self::mean_normalized(dirs),
            AveragingStrategy::MeanNormalizedPercent(percent) => {
                Self::mean_normalized_percent(dirs, percent)
            }
        }
    }

    fn oldest(dirs: &[Vec2]) -> Vec2 {
        dirs.first().copied().unwrap_or(Vec2::ZERO)
    }

    fn newest(dirs: &[Vec2]) -> Vec2 {
        dirs.last().copied().unwrap_or(Vec2::ZERO)
    }

    // FIXME: handle div zero
    fn mean(dirs: &[Vec2]) -> Vec2 {
        dirs.iter().sum::<Vec2>() / dirs.len().max(1) as f32
    }

    //
    fn mean_ignore_zero(dirs: &[Vec2]) -> Vec2 {
        let (sum, n) = dirs.iter().fold((Vec2::ZERO, 0), |(sum, n), &dir| {
            let is_not_zero = (dir != Vec2::ZERO) as usize;
            (sum + dir, n + is_not_zero)
        });
        sum / n.max(1) as f32
    }

    fn mean_normalized(dirs: &[Vec2]) -> Vec2 {
        Self::mean(dirs).normalize_or_zero()
    }

    fn mean_normalized_percent(dirs: &[Vec2], percent: f32) -> Vec2 {
        let mean = Self::mean(dirs);
        let norm = mean.normalize_or_zero();
        // precise because require output to be 0..=1
        lerp_precise_2(mean, norm, percent)
    }
}

impl Display for AveragingStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AveragingStrategy::Oldest => write!(f, "Oldest"),
            AveragingStrategy::Newest => write!(f, "Newest"),
            AveragingStrategy::Mean => write!(f, "Mean"),
            AveragingStrategy::MeanIgnoreZero => write!(f, "MeanIgnoreZero"),
            AveragingStrategy::MeanNormalized => write!(f, "MeanNormalized"),
            AveragingStrategy::MeanNormalizedPercent(percent) => {
                write!(f, "Mean {}% normalized", percent)
            }
        }
    }
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

    pub fn handle_movement(&mut self, wish_dir: Vec2, accel: f32) -> &mut Self {
        // let dir = Self::average_input(desired_dir);
        self.movement.accel = wish_dir * accel;
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

    // // mean. TODO: test bias latest input
    // // TODO: raw mean vs normalized vs mixed
    // // #[allow(clippy::let_and_return)]
    // pub fn average_input(input: &[Vec2]) -> Vec2 {
    //     let mean = input.iter().sum::<Vec2>() / input.len() as f32;

    //     // let norm = mean.normalize_or_zero();
    //     // let ret = lerp_fast2(norm, mean, 0.5);

    //     let ret = mean;
    //     // norm

    //     ret
    // }
}

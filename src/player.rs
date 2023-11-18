use macroquad::prelude::*;

use crate::movement::Movement;
use crate::{DBG_NOW, DBG_PREV, lerp_fast2, lerp_precise_2};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum AveragingStrategy {
    /// Latest
    None,
    /// Mean average
    Mean,
    /// Mean normalized to 1.0
    #[default]
    MeanNormalized,
    // 0 -> mean, 1.0 -> mean normalized, uses lerp
    MeanNormalizedPercent(f32),
}

impl AveragingStrategy {
    // expect about 33 items to average for 1000hz and 10 tps
    pub fn average(&self, dirs: &[Vec2]) -> Vec2 {
        match *self {
            AveragingStrategy::None => Self::none(dirs),
            AveragingStrategy::Mean => Self::mean(dirs),
            AveragingStrategy::MeanNormalized => Self::mean_normalized(dirs),
            AveragingStrategy::MeanNormalizedPercent(percent) => Self::mean_normalized_percent(dirs, percent),
        }
    }

    fn none(dirs: &[Vec2]) -> Vec2 {
        dirs.last().copied().unwrap_or(Vec2::ZERO)
    }

    fn mean(dirs: &[Vec2]) -> Vec2 {
        dirs.iter().sum::<Vec2>() / dirs.len() as f32
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

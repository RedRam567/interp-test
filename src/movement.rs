#![allow(dead_code)] // TODO: just remove them?

// use crate::spline;
// use macroquad::color::*;
use macroquad::math::Vec2;

use super::lerp_fast2;

// "3" types of friction
// static // ignore
// ground friction (relative to normal)
// air resitance
// f = 0.5*desity * v*v * drag coeff * area
// drag coeff = shape and reylond number
// eh most games ignore air resitance (maybe only while on ground)
// just do constant + constant * v (or v*v)
// other form: Pd = Fd
// very slow = v, fast = v*v

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Movement {
    /// current pos
    pub pos: Vec2,
    /// vel needed to get current pos
    pub vel: Vec2,
    /// accel needed to get current vel
    pub accel: Vec2,
    // pub prev_pos: Vec2,
    // pub prev_vel: Vec2,
    // pub prev_accel: Vec2,
}

impl Movement {
    // TODO: movement2 class, with friction builtin
    pub fn step(&mut self, max_speed: f32, base_friction: f32, scaling_friction: f32) -> &mut Self {
        self.apply_friction(base_friction, scaling_friction)
            .step_frictionless(max_speed)
    }

    /// Update player velocity and position
    pub fn step_frictionless(&mut self, max_speed: f32) -> &mut Self {
        self.step_vel().limit_speed(max_speed).step_pos();
        self
    }

    pub(crate) fn apply_friction(&mut self, base: f32, scaling: f32) -> &mut Self {
        let dir = self.vel.normalize_or_zero(); // TODO: doesn't scale with speed factor
        let speed = self.vel.length();
        let amount = (base + scaling * speed).min(speed); // cap to speed so dont go negative
        let friction = dir * amount;

        self.vel -= friction;
        self
    }

    /// increment velocity by acceleration
    pub(crate) fn step_vel(&mut self) -> &mut Self {
        self.vel += self.accel;
        self
    }

    /// increment position by velocity
    pub(crate) fn step_pos(&mut self) -> &mut Self {
        self.pos += self.vel;
        self
    }

    pub fn limit_speed(&mut self, speed: f32) -> &mut Self {
        if self.vel.length() > speed {
            let vel = self.vel.normalize_or_zero() * speed;
            self.vel = vel;
        }
        self
    }

    pub fn interp(&self, next: &Self, t: f32) -> Vec2 {
        // self.interp0(next, t) // linear
        self.interp3(next, t) // curvy

        // horrible:
        // self.interp3_c(next, t)
        // Self::bezier_3(self, next, t)
        // Self::bezier_4(self, next, t)
        // Self::bezier_4b(self, next, t)
    }

    // fn bezier_3(&self, next: &Self, t: f32) -> Vec2 {
    //     // let c1 = self.pos + self.prev_vel / 3.0;
    //     // let c1 = self.pos - self.accel;

    //     // let c1 = next.pos - self.accel;

    //     let mirror = next.pos - self.pos;
    //     let angle = (self.accel).angle_between(mirror);
    //     dbg!(angle);

    //     let mut c1 = (self.pos + next.pos) / 2.0;
    //     crate::dbg_line(self.pos, self.pos + (self.vel.normalize()) * 25.0, MAGENTA);
    //     c1 += self.vel.normalize() * self.accel.length() * (angle);
    //     //asdjihsdayuhsadgyu1
    //     crate::dbg_circle(c1);
    //     // self.vel.ro

    //     spline::bezier_3(self.pos, c1, next.pos, t)
    // }

    // fn bezier_4(&self, next: &Self, t: f32) -> Vec2 {
    //     let mirror = next.pos - self.pos;
    //     // let mirror = self.vel;
    //     // let c1 = self.pos + self.prev_vel / 3.0;
    //     // let c1 = self.pos - self.accel;
    //     let c1 = self.pos + self.prev_accel;
    //     // dbg_circle(c1);
    //     // dbg_circle(c1);
    //     // dbg_circle(c1);
    //     // let c1 = vec2_mirror(c1, mirror, self.pos) + self.pos;
    //     crate::dbg_arrow(self.pos, mirror, MAGENTA);
    //     // crate::dbg_arrow(self.pos, self.vel * 10.0, BROWN);
    //     let angle = (c1 - self.pos).angle_between(mirror);
    //     crate::dbg_line(self.pos, c1, RED);
    //     dbg!(angle);
    //     let c1 = crate::rotate_around_point(c1, self.pos, angle * 2.0);
    //     crate::dbg_line(self.pos, c1, GREEN);
    //     // let c1 = crate::rotate_around_point(c1, self.pos, angle);
    //     // let c1 = (c1 - self.pos).perp().perp().perp() + self.pos;
    //     // dbg_circle(c1);
    //     // let c1 = (c1 - self.pos).rotate(Vec2::new(0.0, -1.0)) + self.pos;

    //     let c2 = next.pos - self.accel;
    //     crate::dbg_circle(c1);
    //     crate::dbg_circle(c2);

    //     // dbg_circle(c2);

    //     // self.vel.ro
    //     spline::bezier_4(self.pos, c1, c2, next.pos, t)
    // }

    // fn bezier_4b(&self, next: &Self, t: f32) -> Vec2 {
    //     let limit_speed = |vec2: Vec2| {
    //         let dir = vec2.length().min(40.0) * vec2.normalize_or_zero();
    //         dbg!(dir.length());
    //         dir
    //     };
    //     let c1 = self.pos + self.vel / 3.0;
    //     // let c2 = next.pos - (next.vel + next.accel) / 3.0;
    //     let c2 = next.pos - limit_speed(next.vel + next.accel) / 3.0;
    //     crate::dbg_circle(c1);
    //     crate::dbg_circle(c2);
    //     spline::bezier_4(self.pos, c1, c2, next.pos, t)
    //     // spline::bezier_3(self.prev_pos, c1, self.pos, t)
    // }

    /// interp pos only
    pub fn interp0(&self, next: &Self, t: f32) -> Vec2 {
        lerp_fast2(self.pos, next.pos, t)
    }

    // d = v0*t + 1/2*a*t^2
    // xf = xo + v0*t + 1/2*a*t^2
    /// interp assume constant velocity
    /// NOTE: staight worse than interp0
    fn interp1(&self, next: &Self, t: f32) -> Vec2 {
        self.pos + next.vel * t
    }

    /// assume constant acceleration
    pub fn interp2(&self, next: &Self, t: f32) -> Vec2 {
        let t_2 = t * t;
        self.pos + next.vel * t + 0.5 * next.accel * t_2
    }

    /// I dont know how this works.
    /// I've tried for like 3 days of studying kinematics, calculas and splines
    /// and fiddling around, this is the best
    pub fn interp3(&self, next: &Self, t: f32) -> Vec2 {
        // let lerp_dumb = |a, b, t| if t <= 0.5 {a} else {b};
        let t_2 = t * t;
        let nt_2 = 1.0 - t_2;
        let accel = next.vel - self.vel; // because Self.accel lies cuz speed gets capped
                                         // dbg!(accel, next.accel);
        let start_accel = 0.5 * accel * t_2;
        let end_accel = -0.5 * accel * nt_2;
        // let accel = lerp_dumb(start_accel, end_accel, t);
        // let accel = lerp_dumb(start_accel, end_accel, 0.0);
        let accel = lerp_fast2(start_accel, end_accel, t);
        // self.pos + next.vel * t + accel
        lerp_fast2(self.pos, next.pos, t) + accel
        // self.pos + accel
        // self.interp2(next, t)
    }

    // fn interp3_c(&self, next: &Self, t: f32) -> Vec2 {
    //     // let lerp_dumb = |a, b, t| if t <= 0.5 {a} else {b};
    //     let t_2 = t * t;
    //     // let nt = 1.0 - t;
    //     // let nt_2 = 1.0 - t_2;

    //     // if self.pos != next.prev_pos {
    //     //     // println!("bruh");
    //     //     panic!()
    //     // }
    //     // if self.vel != next.prev_vel {
    //     //     // println!("bruh");
    //     //     dbg!(self.vel);
    //     //     dbg!(next.prev_vel);
    //     //     panic!()
    //     // }
    //     // let vel = next.prev_vel * t;

    //     let accel = next.vel - self.vel;
    //     let vel = next.prev_vel * t;
    //     // let vel = self.vel * t; // def no
    //     // let vel = next.vel * t;
    //     let accel = 1.0 * accel * t_2;
    //     let calc_pos = self.pos + vel + accel;
    //     lerp_fast2(calc_pos, next.pos, t)
    //     // calc_pos
    // }

    // pub fn interp4(&self, next: &Self, t: f32) -> Vec2 {
    //     // let lerp_dumb = |a, b, t| if t <= 0.5 {a} else {b};
    //     let t_2 = t * t;
    //     let nt_2 = 1.0 - t_2;
    //     let accel = next.vel - self.vel;
    //     let start_accel = 0.5 * accel * t_2 * 2.0;
    //     // let end_accel = -0.5 * next.accel * nt_2;
    //     // let accel = lerp_dumb(start_accel, end_accel, t);
    //     // let accel = lerp_dumb(start_accel, end_accel, 0.0);
    //     // let accel = lerp_fast2(start_accel, end_accel, t) * 1.0;
    //     let accel = start_accel;
    //     self.pos + self.vel * t + accel;
    //     // lerp_fast2(self.pos, next.pos, t) + accel
    //     // self.pos + accel
    //     // self.interp2(next, t)
    //     // self.interp3(next, t)
    // }
}

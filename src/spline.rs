use bezier_nd::Bezier;
use geo_nd::{FArray, Vector};
use macroquad::math::Vec2;

pub fn bezier_4(start: Vec2, c1: Vec2, c2: Vec2, end: Vec2, t: f32) -> Vec2 {
    let start = vec2_to_farray(start);
    let c1 = vec2_to_farray(c1);
    let c2 = vec2_to_farray(c2);
    let end = vec2_to_farray(end);
    let curve = Bezier::cubic(&start, &c1, &c2, &end);
    farray_to_vec2(curve.point_at(t))
}

pub fn bezier_3(start: Vec2, c1: Vec2, end: Vec2, t: f32) -> Vec2 {
    let start = vec2_to_farray(start);
    let c1 = vec2_to_farray(c1);
    let end = vec2_to_farray(end);
    let curve = Bezier::quadratic(&start, &c1, &end);
    farray_to_vec2(curve.point_at(t))
}

fn vec2_to_farray(vec2: Vec2) -> FArray<f32, 2> {
    FArray::from_array([vec2.x, vec2.y])
}

fn farray_to_vec2(arr: FArray<f32, 2>) -> Vec2 {
    let [x, y] = arr.into_array();
    Vec2::new(x, y)
}

// The formula for a 2-points curve:

// P = (1-t)P1 + tP2

// For 3 control points:

// P = (1−t)2P1 + 2(1−t)tP2 + t2P3

// For 4 control points:

// P = (1−t)3P1 + 3(1−t)2tP2 +3(1−t)t2P3 + t3P4

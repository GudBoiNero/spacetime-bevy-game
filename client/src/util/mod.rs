use bevy::math::Vec2;

pub mod actions;

pub fn bool_to_f32(v: bool) -> f32 {
    if v {
        1.0
    } else {
        0.0
    }
}

pub fn nan_to_zero(v: f32) -> f32 {
    if v.is_nan() {
        0.0
    } else {
        v
    }
}

pub fn vec2_nan_to_zero(v: Vec2) -> Vec2 {
    Vec2 {
        x: nan_to_zero(v.x),
        y: nan_to_zero(v.y),
    }
}

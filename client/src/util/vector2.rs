use std::ops::AddAssign;

use crate::module_bindings::StdbVector2;

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Default for Vector2 {
    fn default() -> Self {
        Self {x: 0.0, y: 0.0}
    }
}

impl From<StdbVector2> for Vector2 {
    fn from(value: StdbVector2) -> Self {
        Vector2 { x: value.x, y: value.y }
    }
}

impl From<Vector2> for StdbVector2 {
    fn from(value: Vector2) -> Self {
        StdbVector2 { x: value.x, y: value.y }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
use std::{ops::{AddAssign, Add}};

use crate::module_bindings::StdbVector2;

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn normalized(&self) -> Vector2 {
        let magnitude = f32::sqrt(self.x.powi(2) + self.y.powi(2));
        Self { x: self.x / magnitude, y: self.x / magnitude }
    }
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

impl Add for Vector2 {
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }

    type Output = Self;
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
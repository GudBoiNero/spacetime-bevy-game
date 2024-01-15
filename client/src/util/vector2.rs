use std::{ops::{AddAssign, Add, Mul, MulAssign, Div, DivAssign, Sub, SubAssign}, f32::NAN};

use crate::module_bindings::StdbVector2;

#[derive(Clone, Copy)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32
}

impl Vector2 {
    pub fn normalized(&self) -> Vector2 {
        let magnitude = f32::sqrt(self.x.powi(2) + self.y.powi(2));
        let x = self.x / magnitude;
        let y = self.y / magnitude;
        Self { x: if x.is_nan() {0.0} else {x}, y: if y.is_nan() {0.0} else {y} }
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

impl Sub for Vector2 {
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y
        }
    }

    type Output = Self;
}

impl SubAssign for Vector2 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul for Vector2 {
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y
        }
    }

    type Output = Self;
}

impl MulAssign for Vector2 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Div for Vector2 {
    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y
        }
    }

    type Output = Self;
}

impl DivAssign for Vector2 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}
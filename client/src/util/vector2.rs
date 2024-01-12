use bevy::prelude::Component;

use crate::module_bindings::StdbVector2;

#[derive(Component)]
pub struct Vector2 {
    x: f32,
    y: f32
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
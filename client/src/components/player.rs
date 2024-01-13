use bevy::prelude::*;

use crate::util::vector2::Vector2;

#[derive(Component, Clone, Copy)]
pub struct Player {
    pub position: Vector2,
    pub velocity: Vector2
}

impl Default for Player {
    fn default() -> Self {
        Self { velocity: Vector2 { x: 0.0, y: 1.0 }, position: Vector2 { x: 0.0, y: 0.0 } }
    }
}
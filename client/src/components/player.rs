use bevy::prelude::*;

use crate::util::vector2::Vector2;

use super::velocity::Velocity;

#[derive(Component, Clone)]
pub struct Player {
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self { speed: 8.0 }
    }
}

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub marker: Player,
    pub velocity: Velocity,
    pub sprite: SpriteBundle
}
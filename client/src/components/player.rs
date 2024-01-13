use bevy::prelude::*;

use crate::util::vector2::Vector2;

#[derive(Component, Clone)]
pub struct Player {
    pub position: Vector2,
    pub velocity: Vector2,
    pub sprite: SpriteBundle
}
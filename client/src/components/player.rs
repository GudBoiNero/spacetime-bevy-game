use bevy::prelude::*;

use crate::util::vector2::Vector2;

use super::velocity::Velocity;

#[derive(Component, Clone, Default)]
pub struct Player;

#[derive(Bundle, Default)]
pub struct PlayerBundle {
    pub marker: Player,
    pub velocity: Velocity,
    pub sprite: SpriteBundle
}
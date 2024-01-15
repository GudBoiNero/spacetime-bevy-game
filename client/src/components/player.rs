use super::{velocity::Velocity, owner::Owner};
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Player {
    pub speed: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            speed: 8.0, 
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub marker: Player,
    pub velocity: Velocity,
    pub sprite: SpriteBundle,
    pub owner: Owner
}

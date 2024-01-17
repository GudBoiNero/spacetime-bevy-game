use bevy::{
    ecs::{bundle::Bundle, component::Component},
    math::Vec2,
    sprite::{Sprite, SpriteBundle},
};

use crate::StdbPlayer;

#[derive(Component)]
pub struct Player {
    pub data: StdbPlayer,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_bundle: SpriteBundle,
}

impl PlayerBundle {
    pub fn new(player: Player) -> Self {
        Self {
            player,
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 50.0, y: 50.0 }),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

use std::process::Command;

use bevy::{prelude::*, input};

use crate::{components::{player::{Player, PlayerBundle}, velocity::Velocity}, util::vector2::Vector2, module_bindings::create_player};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, (update_velocity, update_position));
    }
}

fn spawn_player(mut c: Commands) {
    create_player();
    c.spawn(PlayerBundle { 
        sprite: {
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 50.0, y: 50.0}),
                    ..Default::default()
                },
                ..Default::default()
            }
        }, 
        ..Default::default()
    });
}

fn update_velocity(mut q: Query<(&Player, &mut Velocity)>) {
    
}

fn update_position(mut q: Query<(&Player, &mut Velocity, &mut Transform)>) {
    for (_player, velocity, mut transform) in &mut q {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}
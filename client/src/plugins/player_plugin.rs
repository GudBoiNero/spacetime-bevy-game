use std::process::Command;

use bevy::prelude::*;

use crate::{components::{player::{Player, PlayerBundle}, velocity::Velocity}, util::vector2::Vector2, module_bindings::create_player};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, spawn_player)
            .add_systems(Update, update_movement);
    }
}

fn spawn_player(mut c: Commands) {
    create_player();
    c.spawn(PlayerBundle::default());
}

fn update_movement(mut q: Query<(&Player, &mut Velocity, &mut Transform)>) {
    for (_player, velocity, mut transform) in &mut q {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}
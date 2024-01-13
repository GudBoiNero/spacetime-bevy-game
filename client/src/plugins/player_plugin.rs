use bevy::prelude::*;

use crate::{components::player::Player, util::vector2::Vector2};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_movement);
    }
}

fn update_movement(mut q: Query<&mut Player>) {
    for mut player in &mut q {
        let velocity: Vector2 = player.velocity;
        player.as_mut().position += velocity;
        println!("{} : {}", player.position.x, player.position.y);
    }
}
use bevy::ecs::system::Resource;

use crate::components::player::PlayerBundle;

#[derive(Resource)]
pub struct PlayerSpawner {
    pub bundles: Vec<PlayerBundle>
}
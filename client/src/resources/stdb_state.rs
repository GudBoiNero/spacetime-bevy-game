use bevy::ecs::{system::Resource};

use crate::components::player::PlayerBundle;

#[derive(Resource)]
pub struct StdbState {
    pub player_spawner: Vec<PlayerBundle>
}

impl Default for StdbState {
    fn default() -> Self {
        Self {
            player_spawner: Vec::new()
        }
    }
}
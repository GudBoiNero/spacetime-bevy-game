use bevy::ecs::{system::Resource};
use spacetimedb_sdk::identity::Identity;

#[derive(Resource)]
pub struct StdbState {
    pub identities: Vec<Identity>
}

impl Default for StdbState {
    fn default() -> Self {
        Self {
            identities: Vec::new()
        }
    }
}
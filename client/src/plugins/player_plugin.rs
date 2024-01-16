use bevy::{app::{Startup, App, Plugin, Update}, a11y::accesskit::Action, ecs::system::{Commands, ResMut, Res}};

use crate::resources::uncb_receiver::UncbReceiver;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, init_players);
    }
}

/// Grabs all `StdbPlayer`s from the database and spawns \
/// a player for them. If the player has the same `Identity` \
/// as the client, initialize it's bundle with input controls.
fn init_players(c: Commands) {
    
}

/// Grabs all foreign players from the database and \
/// updates their position locally.
fn update_position() {
    
}
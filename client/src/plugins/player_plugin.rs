use bevy::{app::{Startup, App, Plugin, Update}, a11y::accesskit::Action, ecs::system::Commands};

use crate::{UncbRecv, UncbMessage};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

/// Grabs all `StdbPlayer`s from the database and spawns \
/// a player for them. If the player has the same `Identity` \
/// as the client, initialize it's bundle with input controls.
fn init_players(c: Commands) {

}
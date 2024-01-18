use bevy::{
    a11y::accesskit::Action,
    app::{App, Plugin, Startup, Update},
    ecs::{
        system::{Commands, Res, ResMut},
        world::World,
    },
};

use crate::resources::uncb_receiver::{UncbMessage, UncbReceiver};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_players)
            .add_systems(Update, process_messages(&self));
    }
}

/// Process messages
fn process_messages(
    plugin: &PlayerPlugin,
) -> impl FnMut(ResMut<UncbReceiver>, Commands) + Send + 'static {
    move |mut recv, c| {
        for message in recv.get_messages().iter() {
            match message {
                UncbMessage::PlayerInserted { player, event } => {
                    println!("Player inserted!")
                }
                _ => {}
            }
        }
    }
}

/// Grabs all `StdbPlayer`s from the database and spawns \
/// a player for them. If the player has the same `Identity` \
/// as the client, initialize it's bundle with input controls.
fn init_players(c: Commands) {}

/// Grabs all foreign players from the database and \
/// updates their position locally.
fn update_position() {}

/// Specifically spawns one player with the corresponding identity.
fn on_player_inserted() {}

/// Specifically removes one player with the corresponding identity.
fn on_player_removed() {}

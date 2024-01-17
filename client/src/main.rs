use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use spacetimedb_sdk::{
    identity::{load_credentials, once_on_connect, save_credentials, Credentials, Identity},
    on_disconnect, subscribe,
    table::{TableType, TableWithPrimaryKey},
    Address,
};

mod components;
mod module_bindings;
mod plugins;
mod util;

use bevy_tasks::*;
use module_bindings::*;
use plugins::{player_plugin::PlayerPlugin, *};
use util::actions::GameActions;

const SPACETIMEDB_URI: &str = "http://localhost:3000";
const DB_NAME: &str = "spacetime-bevy-game";
const CREDS_DIR: &str = ".spacetime-bevy-game";

fn main() {
    register_callbacks();
    connect_to_db();
    subscribe_to_tables();

    let mut app = App::new();
    app.add_plugins(InputManagerPlugin::<GameActions>::default())
        .add_plugins((DefaultPlugins, PlayerPlugin))
        .add_systems(Startup, init_camera)
        .run();
}

fn init_camera(mut c: Commands) {
    c.spawn(Camera2dBundle {
        ..Default::default()
    });
}

fn connect_to_db() {
    connect(
        SPACETIMEDB_URI,
        DB_NAME,
        load_credentials(CREDS_DIR).expect("Error reading stored credentials"),
    )
    .expect("Failed to connect");
}

/// Register subscriptions for all rows of both tables.
fn subscribe_to_tables() {
    subscribe(&["SELECT * FROM *"]).unwrap();
}

//#region callbacks
fn register_callbacks() {
    once_on_connect(on_connected);
    on_disconnect(on_disconnected);

    StdbClient::on_insert(on_client_inserted);
    StdbClient::on_update(on_client_updated);

    StdbPlayer::on_insert(on_player_inserted);
    StdbPlayer::on_update(on_player_updated);
}

fn on_connected(creds: &Credentials, _client_address: Address) {
    if let Err(e) = save_credentials(CREDS_DIR, creds) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

fn on_disconnected() {
    eprintln!("Disconnected!");
    std::process::exit(0)
}

fn on_client_inserted(client: &StdbClient, _: Option<&ReducerEvent>) {
    if client.connected {
        println!(
            "Client {} connected.",
            identity_leading_hex(&client.client_id)
        );
    }
}

fn on_client_updated(old: &StdbClient, new: &StdbClient, _: Option<&ReducerEvent>) {
    if old.connected && !new.connected {
        println!(
            "User {} disconnected.",
            identity_leading_hex(&new.client_id)
        );
    }
    if !old.connected && new.connected {
        println!("User {} connected.", identity_leading_hex(&new.client_id));
    }
}

fn on_player_inserted(player: &StdbPlayer, _: Option<&ReducerEvent>) {}

fn on_player_updated(old: &StdbPlayer, new: &StdbPlayer, _: Option<&ReducerEvent>) {}

fn identity_leading_hex(id: &Identity) -> String {
    hex::encode(&id.bytes()[0..8])
}
//#endregion callbacks

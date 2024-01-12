use bevy::prelude::*;
use spacetimedb_sdk::{
    Address,
    disconnect,
    identity::{load_credentials, once_on_connect, save_credentials, Credentials, Identity, ConnectCallbackId},
    on_disconnect, on_subscription_applied,
    reducer::Status,
    subscribe,
    table::{TableType, TableWithPrimaryKey},
};

mod module_bindings;
use module_bindings::*;

fn main() {
    register_callbacks();
    connect_to_db();
    subscribe_to_tables();
    user_input_loop();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, update)
        .run();
}


fn init() {

}

fn update() {

}

fn connect_to_db() {

}

//#region subscribers
fn subscribe_to_tables() {

}
//#endregion subscribers

//#region input_loop
fn user_input_loop() {

}
//#endregion input_loop

//#region callbacks
const CREDS_DIR: &str = ".spacetime_chat";

fn register_callbacks() {
    once_on_connect(on_connected);
    
    // When a new user joins, print a notification.
    Client::on_insert(on_client_inserted);

    // When a user's status changes, print a notification.
    Client::on_update(on_client_updated);

    // When our connection closes, inform the user and exit.
    on_disconnect(on_disconnected);
}

fn on_connected(creds: &Credentials, _client_address: Address) {
    if let Err(e) = save_credentials(CREDS_DIR, creds) {
        eprintln!("Failed to save credentials: {:?}", e);
    }
}

fn on_disconnected() {

}

/// Our `User::on_insert` callback:
/// if the user is online, print a notification.
fn on_client_inserted(client: &Client, _: Option<&ReducerEvent>) {
    if client.connected {
        println!("Client {} connected.", identity_leading_hex(&client.client_id));
    }
}

fn identity_leading_hex(id: &Identity) -> String {
    hex::encode(&id.bytes()[0..8])
}

/// Our `User::on_update` callback:
/// print a notification about name and status changes.
fn on_client_updated(old: &Client, new: &Client, _: Option<&ReducerEvent>) {
    if old.connected && !new.connected {
        println!("User {} disconnected.", identity_leading_hex(&new.client_id));
    }
    if !old.connected && new.connected {
        println!("User {} connected.", identity_leading_hex(&new.client_id));
    }
}
//#endregion callbacks
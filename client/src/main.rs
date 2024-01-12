use bevy::{prelude::*, reflect::impl_reflect_struct};
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

mod util;
use util::*;

const SPACETIMEDB_URI: &str = "http://localhost:3000";
const DB_NAME: &str = "spacetime-bevy-game";
const CREDS_DIR: &str = ".spacetime-bevy-game";

fn main() {
    register_callbacks();
    connect_to_db();
    subscribe_to_tables();

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
    connect(
        SPACETIMEDB_URI,
        DB_NAME,
        load_credentials(CREDS_DIR).expect("Error reading stored credentials"),
    )
    .expect("Failed to connect");
}

//#region subscribers
/// Register subscriptions for all rows of both tables.
fn subscribe_to_tables() {
    subscribe(&["SELECT * FROM User;", "SELECT * FROM Message;"]).unwrap();
}
//#endregion subscribers

//#region callbacks
fn register_callbacks() {
    once_on_connect(on_connected);
    on_disconnect(on_disconnected);
    
    Client::on_insert(on_client_inserted);
    Client::on_update(on_client_updated);
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

fn on_client_inserted(client: &Client, _: Option<&ReducerEvent>) {
    if client.connected {
        println!("Client {} connected.", identity_leading_hex(&client.client_id));
    }
}

fn identity_leading_hex(id: &Identity) -> String {
    hex::encode(&id.bytes()[0..8])
}

fn on_client_updated(old: &Client, new: &Client, _: Option<&ReducerEvent>) {
    if old.connected && !new.connected {
        println!("User {} disconnected.", identity_leading_hex(&new.client_id));
    }
    if !old.connected && new.connected {
        println!("User {} connected.", identity_leading_hex(&new.client_id));
    }
}
//#endregion callbacks
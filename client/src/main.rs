use bevy::{prelude::*, utils::futures};
use spacetimedb_sdk::{
    Address,
    identity::{load_credentials, once_on_connect, save_credentials, Credentials, Identity},
    on_disconnect,
    subscribe,
    table::{TableType, TableWithPrimaryKey},
};

mod module_bindings;
mod plugins;

use module_bindings::*;
use plugins::{*, player_plugin::PlayerPlugin};
use futures_channel::mpsc;

const SPACETIMEDB_URI: &str = "http://localhost:3000";
const DB_NAME: &str = "spacetime-bevy-game";
const CREDS_DIR: &str = ".spacetime-bevy-game";

/// Unbound Callback Enum
/// Used to tell our unbounded reciever what \
/// specific event has occured while passing params.
/// [System based on this](https://github.com/clockworklabs/SpacetimeDB/blob/master/crates/sdk/examples/cursive-chat/main.rs#L45)
enum Uncb {
    PlayerUpdate {
        id: Identity
    },
    PlayerInsert {
        id: Identity
    },
    PlayerDelete {
        id: Identity
    }
}

type UncbSend = mpsc::UnboundedSender<Uncb>;
type UncbRecv = mpsc::UnboundedReceiver<Uncb>;

fn main() {
    let (uncb_send, uncb_recv) = mpsc::unbounded::<Uncb>();

    register_callbacks(uncb_send);
    connect_to_db();
    subscribe_to_tables();

    App::new()
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

//#region subscribers
/// Register subscriptions for all rows of both tables.
fn subscribe_to_tables() {
    subscribe(&["SELECT * FROM *"]).unwrap();
}
//#endregion subscribers

//#region callbacks
fn register_callbacks(uncb_send: UncbSend) {
    once_on_connect(on_connected);
    on_disconnect(on_disconnected);
    

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

fn on_client_inserted(uncb_send: UncbSend) -> impl FnMut(&StdbClient, Option<&ReducerEvent>) + Send + 'static {
    move |client, event| { 
        if client.connected {
            println!("Client {} connected.", identity_leading_hex(&client.client_id));
        }
    }
}

fn on_client_updated(uncb_send: UncbSend) -> impl FnMut(&StdbClient, &StdbClient, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, event| {
        if old.connected && !new.connected {
            println!("User {} disconnected.", identity_leading_hex(&new.client_id));
        }
        if !old.connected && new.connected {
            println!("User {} connected.", identity_leading_hex(&new.client_id));
        }
    }
}

fn on_player_inserted(uncb_send: UncbSend) -> impl FnMut(&StdbPlayer, Option<&ReducerEvent>) + Send + 'static {
    move |player, event| {

    } 
}  

fn on_player_updated(uncb_send: UncbSend) -> impl FnMut(&StdbPlayer, &StdbPlayer, Option<&ReducerEvent>) + Send + 'static  {
    move |old, new, event| {
        
    } 
}

fn identity_leading_hex(id: &Identity) -> String {
    hex::encode(&id.bytes()[0..8])
}
//#endregion callbacks
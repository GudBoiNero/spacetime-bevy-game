use std::borrow::Borrow;

use bevy::{prelude::*, utils::futures};
use resources::uncb_receiver::UncbReceiver;
use spacetimedb_sdk::{
    identity::{load_credentials, once_on_connect, save_credentials, Credentials, Identity},
    on_disconnect, subscribe,
    table::{TableType, TableWithPrimaryKey},
    Address,
};

mod module_bindings;
mod plugins;
mod resources;

use futures_channel::mpsc;
use module_bindings::*;
use plugins::{player_plugin::PlayerPlugin, *};

const SPACETIMEDB_URI: &str = "http://localhost:3000";
const DB_NAME: &str = "spacetime-bevy-game";
const CREDS_DIR: &str = ".spacetime-bevy-game";

/// Unbound Callback Message
/// Used to tell our unbounded reciever what \
/// specific event has occured while passing params.
/// [System based on this](https://github.com/clockworklabs/SpacetimeDB/blob/master/crates/sdk/examples/cursive-chat/main.rs#L45)
#[derive(Clone)]
pub enum UncbMessage {
    PlayerInserted {
        player: StdbPlayer,
        event: ReducerEvent,
    },
    PlayerUpdated {
        old: StdbPlayer,
        new: StdbPlayer,
        event: ReducerEvent,
    },
    PlayerDeleted {
        player: StdbPlayer,
        event: ReducerEvent,
    },
}

pub type UncbSend = mpsc::UnboundedSender<UncbMessage>;
pub type UncbRecv = mpsc::UnboundedReceiver<UncbMessage>;

fn main() {
    let (uncb_send, uncb_recv) = mpsc::unbounded();

    register_callbacks(uncb_send.clone());
    connect_to_db();
    subscribe_to_tables();

    let mut app = App::new();
    app.insert_resource(UncbReceiver::new(uncb_recv))
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

    StdbClient::on_insert(on_client_inserted(uncb_send.clone()));
    StdbClient::on_update(on_client_updated(uncb_send.clone()));

    StdbPlayer::on_insert(on_player_inserted(uncb_send.clone()));
    StdbPlayer::on_update(on_player_updated(uncb_send.clone()));
    StdbPlayer::on_delete(on_player_deleted(uncb_send.clone()));
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

fn on_client_inserted(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbClient, Option<&ReducerEvent>) + Send + 'static {
    move |client, event| {
        if client.connected {
            println!(
                "Client {} connected.",
                identity_leading_hex(&client.client_id)
            );
        }
    }
}

fn on_client_updated(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbClient, &StdbClient, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, event| {
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
}

fn on_player_inserted(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbPlayer, Option<&ReducerEvent>) + Send + 'static {
    move |player, event| {
        if let Some(event) = event {
            uncb_send
                .start_send(UncbMessage::PlayerInserted {
                    player: player.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_player_updated(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbPlayer, &StdbPlayer, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, event| {
        if let Some(event) = event {
            uncb_send
                .start_send(UncbMessage::PlayerUpdated {
                    old: old.clone(),
                    new: new.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_player_deleted(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbPlayer, Option<&ReducerEvent>) + Send + 'static {
    move |player, event| {
        if let Some(event) = event {
            uncb_send
                .start_send(UncbMessage::PlayerDeleted {
                    player: player.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn identity_leading_hex(id: &Identity) -> String {
    hex::encode(&id.bytes()[0..8])
}
//#endregion callbacks

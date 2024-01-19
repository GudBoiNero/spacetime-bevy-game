use std::borrow::Borrow;

use bevy::{prelude::*, utils::futures};
use leafwing_input_manager::plugin::InputManagerPlugin;
use resources::uncb_receiver::{
    process_messages, UncbEvent, UncbMessage, UncbReceiver, UncbRecv, UncbSend,
};
use spacetimedb_sdk::{
    identity::{load_credentials, once_on_connect, save_credentials, Credentials, Identity},
    on_disconnect, subscribe,
    table::{TableType, TableWithPrimaryKey},
    Address,
};

mod components;
mod module_bindings;
mod plugins;
mod resources;
mod util;

use futures_channel::mpsc;
use module_bindings::*;
use plugins::{player_plugin::PlayerPlugin, *};
use util::actions::GameActions;

const SPACETIMEDB_URI: &str = "http://localhost:3000";
const DB_NAME: &str = "spacetime-bevy-game";
const CREDS_DIR: &str = ".spacetime-bevy-game";
const DEBUG_MODE: bool = false;

fn main() {
    let (uncb_send, uncb_recv) = mpsc::unbounded();

    register_callbacks(uncb_send.clone());
    connect_to_db();
    subscribe_to_tables();

    let mut app = App::new();
    app.insert_resource(UncbReceiver::new(uncb_recv))
        .add_event::<UncbEvent>()
        .add_plugins((
            DefaultPlugins,
            PlayerPlugin,
            InputManagerPlugin::<GameActions>::default(),
        ))
        .add_systems(Startup, init_camera)
        .add_systems(Update, process_messages)
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
        if DEBUG_MODE {
            None
        } else {
            load_credentials(CREDS_DIR).expect("Error reading stored credentials")
        },
    )
    .expect("Failed to connect");
}

/// Register subscriptions for all rows of tables.
fn subscribe_to_tables() {
    subscribe(&["SELECT * FROM *"]).unwrap();
}

//#region callbacks
fn register_callbacks(uncb_send: UncbSend) {
    once_on_connect(on_connected);
    on_disconnect(on_disconnected);

    StdbObject::on_insert(on_object_inserted(uncb_send.clone()));
    StdbObject::on_update(on_object_updated(uncb_send.clone()));
    StdbObject::on_delete(on_object_deleted(uncb_send.clone()));

    StdbClient::on_insert(on_client_inserted(uncb_send.clone()));
    StdbClient::on_update(on_client_updated(uncb_send.clone()));
    StdbClient::on_delete(on_client_deleted(uncb_send.clone()));

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

fn on_object_inserted(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbObject, Option<&ReducerEvent>) + Send + 'static {
    move |object, event| {
        if let Some(event) = event {
            uncb_send
                .start_send(UncbMessage::ObjectInserted {
                    data: object.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_object_updated(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbObject, &StdbObject, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, event| {
        if let Some(event) = event {
            uncb_send
                .start_send(UncbMessage::ObjectUpdated {
                    new: new.clone(),
                    old: old.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_object_deleted(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbObject, Option<&ReducerEvent>) + Send + 'static {
    move |object, event| {
        if let Some(event) = event {
            uncb_send
                .start_send(UncbMessage::ObjectRemoved {
                    data: object.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
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

fn on_client_deleted(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbClient, Option<&ReducerEvent>) + Send + 'static {
    move |client, event| {}
}

fn on_player_inserted(
    mut uncb_send: UncbSend,
) -> impl FnMut(&StdbPlayer, Option<&ReducerEvent>) + Send + 'static {
    move |player, event| {
        if let Some(event) = event {
            uncb_send
                .start_send(UncbMessage::PlayerInserted {
                    data: player.clone(),
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
                .start_send(UncbMessage::PlayerRemoved {
                    data: player.clone(),
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

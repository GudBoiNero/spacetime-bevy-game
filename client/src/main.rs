use actions::GameActions;
use bevy::math::Vec2;
use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use player_plugin::PlayerPlugin;
use spacetimedb_sdk::{
    identity::{load_credentials, once_on_connect, save_credentials, Credentials, Identity},
    on_disconnect, subscribe,
    table::{TableType, TableWithPrimaryKey},
    Address,
};

mod actions;
mod module_bindings;
mod player;
mod player_plugin;
mod uncb_receiver;

use futures_channel::mpsc;
use module_bindings::*;
use uncb_receiver::{process_messages, UncbEvent, UncbMessage, UncbReceiver, UncbSend};

const SPACETIMEDB_URI: &str = "http://localhost:3000";
const DB_NAME: &str = "spacetime-bevy-game";
const CREDS_DIR: &str = ".spacetime-bevy-game";
const DEBUG_MODE: bool = true;

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
    once_on_connect(on_connected(uncb_send.clone()));
    on_disconnect(on_disconnected(uncb_send.clone()));

    StdbObject::on_insert(on_object_inserted(uncb_send.clone()));
    StdbObject::on_update(on_object_updated(uncb_send.clone()));
    StdbObject::on_delete(on_object_deleted(uncb_send.clone()));

    StdbClient::on_insert(on_client_inserted(uncb_send.clone()));
    StdbClient::on_update(on_client_updated(uncb_send.clone()));

    StdbPlayer::on_insert(on_player_inserted(uncb_send.clone()));
    StdbPlayer::on_update(on_player_updated(uncb_send.clone()));
    StdbPlayer::on_delete(on_player_deleted(uncb_send.clone()));
}

fn on_connected(uncb_send: UncbSend) -> impl FnMut(&Credentials, Address) + Send + 'static {
    move |creds, address| {
        if let Err(e) = save_credentials(CREDS_DIR, creds) {
            eprintln!("Failed to save credentials: {:?}", e);
        }
        uncb_send
            .unbounded_send(UncbMessage::Connected {
                creds: creds.clone(),
                address,
            })
            .unwrap();
    }
}

fn on_disconnected(uncb_send: UncbSend) -> impl FnMut() + Send + 'static {
    move || {
        eprintln!("Disconnected!");
        uncb_send.unbounded_send(UncbMessage::Disconnected).unwrap();
        std::process::exit(0)
    }
}

fn on_object_inserted(
    uncb_send: UncbSend,
) -> impl FnMut(&StdbObject, Option<&ReducerEvent>) + Send + 'static {
    move |object, event| {
        if let Some(event) = event {
            uncb_send
                .unbounded_send(UncbMessage::ObjectInserted {
                    data: object.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_object_updated(
    uncb_send: UncbSend,
) -> impl FnMut(&StdbObject, &StdbObject, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, event| {
        if let Some(event) = event {
            uncb_send
                .unbounded_send(UncbMessage::ObjectUpdated {
                    new: new.clone(),
                    old: old.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_object_deleted(
    uncb_send: UncbSend,
) -> impl FnMut(&StdbObject, Option<&ReducerEvent>) + Send + 'static {
    move |object, event| {
        if let Some(event) = event {
            uncb_send
                .unbounded_send(UncbMessage::ObjectRemoved {
                    data: object.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_client_inserted(
    _uncb_send: UncbSend,
) -> impl FnMut(&StdbClient, Option<&ReducerEvent>) + Send + 'static {
    move |client, _event| {
        if client.connected {
            println!(
                "Client {} connected.",
                identity_leading_hex(&client.client_id)
            );
        }
    }
}

fn on_client_updated(
    mut _uncb_send: UncbSend,
) -> impl FnMut(&StdbClient, &StdbClient, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, _event| {
        if old.connected && !new.connected {
            println!(
                "Client {} disconnected.",
                identity_leading_hex(&new.client_id)
            );
        }
        if !old.connected && new.connected {
            println!("Client {} connected.", identity_leading_hex(&new.client_id));
        }
    }
}

fn on_player_inserted(
    uncb_send: UncbSend,
) -> impl FnMut(&StdbPlayer, Option<&ReducerEvent>) + Send + 'static {
    move |player, event| {
        if let Some(event) = event {
            info!("UncbMessage::PlayerInserted called");
            uncb_send
                .unbounded_send(UncbMessage::PlayerInserted {
                    data: player.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_player_updated(
    uncb_send: UncbSend,
) -> impl FnMut(&StdbPlayer, &StdbPlayer, Option<&ReducerEvent>) + Send + 'static {
    move |old, new, event| {
        if let Some(event) = event {
            info!("UncbMessage::PlayerUpdated called");
            uncb_send
                .unbounded_send(UncbMessage::PlayerUpdated {
                    old: old.clone(),
                    new: new.clone(),
                    event: event.clone(),
                })
                .unwrap();
        }
    }
}

fn on_player_deleted(
    uncb_send: UncbSend,
) -> impl FnMut(&StdbPlayer, Option<&ReducerEvent>) + Send + 'static {
    move |player, _event| {
        info!("UncbMessage::PlayerRemoved called");
        uncb_send
            .unbounded_send(UncbMessage::PlayerRemoved {
                data: player.clone(),
            })
            .unwrap();
    }
}

fn identity_leading_hex(id: &Identity) -> String {
    hex::encode(&id.bytes()[0..8])
}
//#endregion callbacks

//#region helpers
pub fn bool_to_f32(v: bool) -> f32 {
    if v {
        1.0
    } else {
        0.0
    }
}

pub fn nan_to_zero(v: f32) -> f32 {
    if v.is_nan() {
        0.0
    } else {
        v
    }
}

pub fn vec2_nan_to_zero(v: Vec2) -> Vec2 {
    Vec2 {
        x: nan_to_zero(v.x),
        y: nan_to_zero(v.y),
    }
}
//#endregion helpers

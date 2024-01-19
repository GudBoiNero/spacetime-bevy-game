use bevy::{
    ecs::{
        event::Event,
        system::{Commands, ResMut, Resource},
        world::World,
    },
    log::info,
};
use futures_channel::mpsc;
use spacetimedb_sdk::{identity::Credentials, Address};

use crate::{ReducerEvent, StdbObject, StdbPlayer};

/// Unbound Callback Message
/// Used to tell our unbounded reciever what \
/// specific event has occured while passing params.
/// [System based on this](https://github.com/clockworklabs/SpacetimeDB/blob/master/crates/sdk/examples/cursive-chat/main.rs#L45)
#[derive(Clone)]
pub enum UncbMessage {
    Connected {
        creds: Credentials,
        address: Address,
    },
    Disconnected,

    PlayerInserted {
        data: StdbPlayer,
        event: ReducerEvent,
    },
    PlayerUpdated {
        old: StdbPlayer,
        new: StdbPlayer,
        event: ReducerEvent,
    },
    PlayerRemoved {
        data: StdbPlayer,
    },
    ObjectInserted {
        data: StdbObject,
        event: ReducerEvent,
    },
    ObjectUpdated {
        old: StdbObject,
        new: StdbObject,
        event: ReducerEvent,
    },
    ObjectRemoved {
        data: StdbObject,
        event: ReducerEvent,
    },
}

pub type UncbSend = mpsc::UnboundedSender<UncbMessage>;
pub type UncbRecv = mpsc::UnboundedReceiver<UncbMessage>;

#[derive(Resource)]
pub struct UncbReceiver {
    pub recv: UncbRecv,
}

impl UncbReceiver {
    pub fn new(recv: UncbRecv) -> Self {
        UncbReceiver { recv }
    }
}

#[derive(Event)]
pub struct UncbEvent {
    pub message: UncbMessage,
}

/// Any system that requires an `EventReader` of type `UncbEvent` will not function on the `Startup` schedule.
/// \
/// This is because of how `process_messages` is called, it should always be called in the `main` function
/// on the `Update` schedule.
/// Since `Startup` is scheduled to run before and `Update` events, the events will never
/// be present if read with an `EventReader` on `Startup`.
pub fn process_messages(mut res: ResMut<UncbReceiver>, mut c: Commands) {
    loop {
        let message = res.recv.try_next();
        if let Ok(message) = message {
            if let Some(message) = message {
                c.add(|w: &mut World| w.send_event(UncbEvent { message }));
            }
        } else {
            break;
        }
    }
}

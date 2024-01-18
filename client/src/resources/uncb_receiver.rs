use bevy::ecs::{
    event::Event,
    system::{Commands, ResMut, Resource},
    world::World,
};
use futures_channel::mpsc;

use crate::{ReducerEvent, StdbObject, StdbPlayer};

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
    ObjectInserted {
        object: StdbObject,
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
    message: UncbMessage,
}

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

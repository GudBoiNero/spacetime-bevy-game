use bevy::ecs::system::{Commands, ResMut, Resource};
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
    pub messages: Vec<UncbMessage>,
}

impl UncbReceiver {
    pub fn new(recv: UncbRecv) -> Self {
        UncbReceiver {
            recv,
            messages: Vec::new(),
        }
    }

    pub fn get_messages(&mut self) -> &Vec<UncbMessage> {
        &self.messages
    }
}

pub fn process_messages(mut res: ResMut<UncbReceiver>, c: Commands) {
    loop {
        let message = res.recv.try_next();
        if let Ok(message) = message {
            if let Some(message) = message {
                res.messages.push(message);
            }
        } else {
            break;
        }
    }
}

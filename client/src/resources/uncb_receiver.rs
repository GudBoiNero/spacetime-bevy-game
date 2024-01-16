use bevy::ecs::system::Resource;

use crate::{UncbMessage, UncbRecv};

#[derive(Resource)]
pub struct UncbReceiver {
    recv: UncbRecv,
    messages: Vec<UncbMessage>,
}

impl UncbReceiver {
    pub fn new(recv: UncbRecv) -> Self {
        UncbReceiver {
            recv,
            messages: Vec::new(),
        }
    }

    pub fn get_messages(&mut self) -> Vec<UncbMessage> {
        self.refresh_messages().messages.clone()
    }

    fn refresh_messages(&mut self) -> &Self {
        loop {
            let message = self.recv.try_next();
            if let Ok(message) = message {
                if let Some(message) = message {
                    self.messages.push(message);
                }
            } else {
                break;
            }
        }

        self
    }
}

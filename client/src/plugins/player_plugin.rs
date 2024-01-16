use bevy::{app::{Startup, App, Plugin, Update}, a11y::accesskit::Action, ecs::system::Commands};

use crate::{UncbRecv, UncbListener, UncbMessage};

#[derive(Default)]
pub struct PlayerPlugin {
    pub messages: Vec<UncbMessage>
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

impl UncbListener for PlayerPlugin {
    fn send_message(&mut self, message: crate::UncbMessage) {
        self.messages.push(message);
    }
}
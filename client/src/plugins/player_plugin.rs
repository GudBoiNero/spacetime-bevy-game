use bevy::{app::{Startup, App, Plugin, Update}, a11y::accesskit::Action, ecs::system::Commands};

use crate::{UncbSend, UncbRecv};

pub struct PlayerPlugin {
    pub uncb_send: UncbSend
}
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

use bevy::{app::{Startup, App, Plugin, Update}, a11y::accesskit::Action, ecs::system::Commands};

use crate::{UncbRecv, UncbMessage};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        
    }
}
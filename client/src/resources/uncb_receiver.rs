use bevy::ecs::system::{Resource};

use crate::{UncbMessage, UncbRecv};

#[derive(Resource)]
pub struct UncbReceiver {
    pub recv: UncbRecv,
    pub messages: Vec<UncbMessage>,
}
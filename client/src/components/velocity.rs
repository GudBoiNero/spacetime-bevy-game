use bevy::prelude::Component;

use crate::util::vector2::Vector2;

#[derive(Component, Default)]
pub struct Velocity(pub Vector2);
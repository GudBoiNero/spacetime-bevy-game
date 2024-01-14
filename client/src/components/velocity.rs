use bevy::{prelude::Component, ecs::component::{ComponentStorage, TableStorage}};

use crate::util::vector2::Vector2;

#[derive(Component, Default)]
pub struct Velocity(pub Vector2);
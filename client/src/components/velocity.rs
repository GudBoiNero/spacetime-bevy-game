use bevy::{prelude::Component, ecs::component::{ComponentStorage, TableStorage}, a11y::accesskit::Vec2};

#[derive(Component, Default)]
pub struct Velocity(pub Vec2);
use std::process::Command;

use bevy::{input, prelude::*};
use leafwing_input_manager::{input_map::InputMap, Actionlike, InputManagerBundle, action_state::ActionState};

use crate::{
    components::{
        player::{Player, PlayerBundle},
        velocity::Velocity,
    },
    module_bindings::create_player,
    util::vector2::Vector2,
};
pub struct PlayerPlugin;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum Action {
    W,
    A,
    S,
    D,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (update_velocity, update_position));
    }
}

fn spawn_player(mut c: Commands) {
    create_player();
    c.spawn(InputManagerBundle::<Action> {
        action_state: ActionState::default(),
        input_map: InputMap::new([
            (KeyCode::W, Action::W),
            (KeyCode::A, Action::A),
            (KeyCode::S, Action::S),
            (KeyCode::D, Action::D),
        ]),
    })
    .insert(PlayerBundle {
        sprite: {
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 50.0, y: 50.0 }),
                    ..Default::default()
                },
                ..Default::default()
            }
        },
        ..Default::default()
    });
}

fn update_velocity(mut q: Query<(&mut Velocity, &ActionState<Action>), With<Player>>) {
    
}

fn update_position(mut q: Query<(&mut Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in &mut q {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}

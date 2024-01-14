use std::process::Command;

use bevy::{input, prelude::*};
use leafwing_input_manager::{input_map::InputMap, Actionlike, InputManagerBundle, action_state::ActionState, plugin::InputManagerPlugin};

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
            .add_systems(Update, (update_velocity, update_position))
            .add_plugins(InputManagerPlugin::<Action>::default());
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

fn get_input_vector(action: &ActionState<Action>) -> Vector2 {
    Vector2 {
        x: (if action.pressed(Action::D) {1.0} else {0.0}) - (if action.pressed(Action::A) {1.0} else {0.0}),
        y: (if action.pressed(Action::W) {1.0} else {0.0}) - (if action.pressed(Action::S) {1.0} else {0.0})
    }
}

fn update_velocity(mut q: Query<(&mut Velocity, &ActionState<Action>), With<Player>>) {
    let (mut velocity, action) = q.single();
    let input_vector = get_input_vector(action);
    println!("{}, {}", input_vector.x, input_vector.y);
}

fn update_position(mut q: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in &mut q {
        transform.translation.x += velocity.0.x;
        transform.translation.y += velocity.0.y;
    }
}

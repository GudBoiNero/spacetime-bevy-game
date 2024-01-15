use std::process::Command;

use bevy::{input, prelude::*, a11y::accesskit::Vec2};
use leafwing_input_manager::{input_map::InputMap, Actionlike, InputManagerBundle, action_state::ActionState, plugin::InputManagerPlugin};

use crate::{
    components::{
        player::{Player, PlayerBundle},
    },
    module_bindings::create_player, util::{vec2::normalized, conversions::f64_to_f32},
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
            .add_systems(Update, update_position)
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
                    custom_size: Some(bevy::math::Vec2 { x: 50.0, y: 50.0 }),
                    ..Default::default()
                },
                ..Default::default()
            }
        },
        ..Default::default()
    });
}

fn get_input_vector(action: &ActionState<Action>) -> Vec2 {
    Vec2 {
        x: (if action.pressed(Action::D) {1.0} else {0.0}) - (if action.pressed(Action::A) {1.0} else {0.0}),
        y: (if action.pressed(Action::W) {1.0} else {0.0}) - (if action.pressed(Action::S) {1.0} else {0.0})
    }
}

fn update_position(mut q: Query<(&Player, &ActionState<Action>, &mut Transform), With<Player>>) {
    for (player, action, mut transform) in &mut q {
        let input_vector = normalized(get_input_vector(action));
        
        transform.translation.x += f64_to_f32(input_vector.x) * player.speed;
        transform.translation.y += f64_to_f32(input_vector.y) * player.speed;

        println!("New Velocity: {}, {}", input_vector.x, input_vector.y)
    }
}

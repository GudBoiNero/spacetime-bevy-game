use std::borrow::BorrowMut;

use bevy::{prelude::*, a11y::accesskit::Vec2};
use leafwing_input_manager::{input_map::InputMap, Actionlike, InputManagerBundle, action_state::ActionState, plugin::InputManagerPlugin};
use spacetimedb_sdk::{identity::Identity, Address, reducer::Status, table::TableType};

use crate::{
    components::{player::{Player, PlayerBundle}, owner::Owner},
    util::{vec2::normalized, conversions::f64_to_f32}, resources::stdb_state::StdbState, module_bindings::StdbPlayer,
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
        app.add_systems(Startup, (read_players, spawn_players))
            .add_systems(Update, (read_players, update_position))
            .add_plugins(InputManagerPlugin::<Action>::default());
    }
}

fn read_players(mut c: Commands, mut res: Option<ResMut<StdbState>>) {
    if StdbPlayer::count() == 0 { return; }

    if let Some(res) = &mut res {    
        for player in StdbPlayer::iter() {
            let len = res.identities.len();
            res.identities.insert(len, player.client_id);
        }
    }
}

fn spawn_players(mut c: Commands, mut res: Res<StdbState>) {
    let client_id = spacetimedb_sdk::identity::identity().expect("Could not get Identity.");
    for id in res.identities.iter() {
        let player_bundle = PlayerBundle {
            sprite: {
                SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(bevy::math::Vec2 { x: 50.0, y: 50.0 }),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            },
            owner: Owner { id: id.clone() },
            marker: default(),
            velocity: default(),
        };
        if *id == client_id {
            c.spawn(InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::new([
                    (KeyCode::W, Action::W),
                    (KeyCode::A, Action::A),
                    (KeyCode::S, Action::S),
                    (KeyCode::D, Action::D),
                ]),
            }).insert(player_bundle);
            continue;
        }

        
    }
}

fn get_input_vector(action: &ActionState<Action>) -> Vec2 {
    Vec2 {
        x: (if action.pressed(Action::D) {1.0} else {0.0}) - (if action.pressed(Action::A) {1.0} else {0.0}),
        y: (if action.pressed(Action::W) {1.0} else {0.0}) - (if action.pressed(Action::S) {1.0} else {0.0})
    }
}

fn update_position(mut q: Query<(&Player, &ActionState<Action>, &mut Transform), With<Player>>) {
    // Add a check to use separate logic for players that are not under ownership of this client.
    for (player, action, mut transform) in &mut q {
        let input_vector = normalized(get_input_vector(action));
        
        transform.translation.x += f64_to_f32(input_vector.x) * player.speed;
        transform.translation.y += f64_to_f32(input_vector.y) * player.speed;

        println!("New Velocity: {}, {}", input_vector.x, input_vector.y)
    }
}

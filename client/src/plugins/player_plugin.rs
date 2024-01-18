use bevy::{
    a11y::accesskit::Action,
    app::{App, Plugin, Startup, Update},
    ecs::{
        query::With,
        system::{Commands, Query, Res, ResMut},
        world::World,
    },
    input::keyboard::KeyCode,
    log::info,
    transform::components::Transform,
};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap, InputManagerBundle};
use spacetimedb_sdk::table::TableType;

use crate::{
    components::player::{Player, PlayerBundle},
    identity_leading_hex,
    resources::uncb_receiver::{UncbMessage, UncbReceiver},
    update_player_pos,
    util::{
        actions::{get_input_vector, GameActions},
        vec2_nan_to_zero,
    },
    StdbObject, StdbPlayer,
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (player_movement, refresh_players, update_players));
    }
}

/// Finds all currently spawned `Player`s and all `StdbPlayer`s within the database. \
/// Spawns only the `StdbPlayer`s that do not have a spawned `Player` with a corresponding `Identity`. \
/// Adds an `InputManagerBundle::<GameActions>` bundle to the *local* `Player` bundle.
fn refresh_players(mut c: Commands, q: Query<&Player>) {
    let mut spawnable_players: Vec<StdbPlayer> = Vec::new();
    'stdb_loop: for stdb_player in StdbPlayer::iter() {
        for player in q.iter() {
            if player.data.client_id == stdb_player.client_id {
                continue 'stdb_loop;
            }
        }
        spawnable_players.push(stdb_player);
    }

    for spawn in spawnable_players {
        info!("Spawned player: {}", identity_leading_hex(&spawn.client_id));
        let bundle = PlayerBundle::new(Player {
            data: spawn.clone(),
        });

        if spawn.client_id == spacetimedb_sdk::identity::identity().unwrap() {
            c.spawn(bundle).insert(InputManagerBundle::<GameActions> {
                // Stores "which actions are currently pressed"
                action_state: ActionState::default(),
                // Describes how to convert from player inputs into those actions
                input_map: InputMap::new([
                    (KeyCode::W, GameActions::W),
                    (KeyCode::A, GameActions::A),
                    (KeyCode::S, GameActions::S),
                    (KeyCode::D, GameActions::D),
                ]),
            });
        } else {
            c.spawn(bundle);
        }
    }
}

/// Grabs all foreign players from the database and \
/// updates their position locally.
fn update_players(
    mut q: Query<
        (
            Option<&ActionState<GameActions>>,
            &mut Transform,
            &mut Player,
        ),
        With<Player>,
    >,
) {
}

fn player_movement(
    mut q: Query<
        (
            Option<&ActionState<GameActions>>,
            &mut Transform,
            &mut Player,
        ),
        With<Player>,
    >,
) {
    for (action_state, mut transform, player) in &mut q {
        // We have a handle to the local player.
        if let Some(action_state) = action_state {
            // Handle input and update transform locally.
            let input_vector = vec2_nan_to_zero(get_input_vector(action_state).normalize());
            transform.translation.x += input_vector.x;
            transform.translation.y += input_vector.y;
            // Then sync to the database.
            update_player_pos(crate::StdbVector2 {
                x: transform.translation.x,
                y: transform.translation.y,
            })
        }
    }
}

/// Specifically spawns one player with the corresponding identity.
fn on_player_inserted() {}

/// Specifically removes one player with the corresponding identity.
fn on_player_removed() {}

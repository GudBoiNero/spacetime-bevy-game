use std::borrow::BorrowMut;

use bevy::{
    a11y::accesskit::Action,
    app::{App, Plugin, Startup, Update},
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
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
    components::player::{Player, PlayerBundle, PLAYER_SPEED},
    create_player, identity_leading_hex,
    resources::uncb_receiver::{UncbEvent, UncbMessage, UncbReceiver},
    stdb_player, update_player_pos,
    util::{
        actions::{get_input_vector, GameActions},
        vec2_nan_to_zero,
    },
    StdbObject, StdbPlayer,
};

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_players)).add_systems(
            Update,
            (
                player_movement,
                insert_players,
                update_players,
                remove_players,
            ),
        );
    }
}

/// Finds all currently spawned `Player`s and all `StdbPlayer`s within the database. \
/// Spawns only the `StdbPlayer`s that do not have a spawned `Player` with a corresponding `Identity`. \
/// Adds an `InputManagerBundle::<GameActions>` bundle to the *local* `Player` bundle.
fn init_players(mut c: Commands, q: Query<&Player>) {
    create_player();
    let mut spawnable_players: Vec<StdbPlayer> = Vec::new();
    'stdb_loop: for stdb_player in StdbPlayer::iter() {
        for player in q.iter() {
            info!(
                "Found player: {}",
                identity_leading_hex(&stdb_player.client_id)
            );
            if player.data.client_id == stdb_player.client_id {
                continue 'stdb_loop;
            }
        }
        spawnable_players.push(stdb_player);
    }

    for spawn in spawnable_players {
        let bundle = PlayerBundle::new(Player {
            data: spawn.clone(),
        });

        if spawn.client_id != spacetimedb_sdk::identity::identity().unwrap() {
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
            info!(
                "Spawned local player: {}",
                identity_leading_hex(&spawn.client_id)
            );
        } else {
            c.spawn(bundle);
            info!(
                "Spawned online player: {}",
                identity_leading_hex(&spawn.client_id)
            );
        }
    }
}

/// Checks for `UncbMessage::PlayerUpdated` message and updates players accordingly.
fn update_players(
    mut q: Query<(&mut Transform, &mut Player), With<Player>>,
    mut er: EventReader<UncbEvent>,
) {
    for (mut transform, player) in &mut q {
        // We have a handle to an online player.
        for ev in er.read() {
            match &ev.message {
                UncbMessage::ObjectUpdated { old, new, event } => {
                    // Read from database and update transform.
                    let stdb_object = StdbObject::filter_by_object_id(player.data.object_id);
                    let position = stdb_object.unwrap().position;
                    transform.translation.x = position.x;
                    transform.translation.y = position.y;
                }
                _ => {}
            }
        }
    }
}

/// Listens for a `UncbMessage::PlayerInserted` message and spawns a `PlayerBundle`.
fn insert_players(mut c: Commands, mut er: EventReader<UncbEvent>) {
    // We have a handle to an online player.
    for ev in er.read() {
        match &ev.message {
            UncbMessage::PlayerInserted {
                data: player,
                event,
            } => {
                let bundle = PlayerBundle::new(Player {
                    data: player.clone(),
                });

                c.spawn(bundle);
                info!(
                    "Player spawned: {}",
                    identity_leading_hex(&player.client_id)
                )
            }
            _ => {}
        }
    }
}

/// Listens for a `UncbMessage::PlayerRemoved` message and removes a `PlayerBundle`.
fn remove_players(
    mut c: Commands,
    mut q: Query<Entity, With<Player>>,
    mut er: EventReader<UncbEvent>,
) {
    for (entity) in &mut q {
        // We have a handle to an online player.
        for ev in er.read() {
            match &ev.message {
                UncbMessage::PlayerRemoved {
                    data: player,
                    event,
                } => {
                    c.entity(entity).remove::<PlayerBundle>();
                    info!(
                        "Player removed: {}",
                        identity_leading_hex(&player.client_id)
                    )
                }
                _ => {}
            }
        }
    }
}

/// Used for local player movement.
fn player_movement(
    mut q: Query<(Option<&ActionState<GameActions>>, &mut Transform), With<Player>>,
) {
    for (action_state, mut transform) in &mut q {
        // We have a handle to the local player.
        if let Some(action_state) = action_state {
            // Handle input and update transform locally.
            let input_vector = vec2_nan_to_zero(get_input_vector(action_state).normalize());
            transform.translation.x += input_vector.x * PLAYER_SPEED;
            transform.translation.y += input_vector.y * PLAYER_SPEED;
            // Then sync to the database.
            update_player_pos(crate::StdbVector2 {
                x: transform.translation.x,
                y: transform.translation.y,
            })
        }
    }
}

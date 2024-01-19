use std::{
    borrow::BorrowMut,
    time::{Duration, SystemTime},
};

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
    time::Time,
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
        app.add_systems(Startup, (create_player,)).add_systems(
            Update,
            (
                refresh_players,
                update_players,
                init_players,
                remove_players,
            ),
        );
    }
}

/// Listens for the `UncbMessage::ObjectRemoved` message and removes the corresponding player's bundle with the same `object_id` locally.
fn remove_players(
    mut c: Commands,
    mut q: Query<(Entity, &Player)>,
    mut er: EventReader<UncbEvent>,
) {
    for ev in er.read() {
        match &ev.message {
            UncbMessage::PlayerRemoved { data, event } => {
                info!("Player removed: {}", data.object_id);
                for (entity, player) in q.iter() {
                    if player.data.object_id == data.object_id {
                        c.entity(entity).remove::<PlayerBundle>();
                    }
                }
            }

            _ => {}
        }
    }
}

/// For every player within a query it checks if the player has an input manager, if it does
/// then it knows we have the client, now we can update it's movement, otherwise, we know we
/// have another player from the database, so instead it reads data from the database and updates those players.
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
        // We have a handle to an online player.
        else {
            // Read from database and update transform.
            let stdb_object = StdbObject::filter_by_object_id(player.data.object_id);
            let position = stdb_object.unwrap().position;
            transform.translation.x = position.x * PLAYER_SPEED;
            transform.translation.y = position.y * PLAYER_SPEED;
        }
    }
}

/// Waits for the `UncbMessage::Connected` message in order to spawn all
/// players that were in the database before connection.
fn init_players(mut c: Commands, mut er: EventReader<UncbEvent>) {
    for ev in er.read() {
        match &ev.message {
            UncbMessage::Connected { creds, address } => {
                for stdb_player in StdbPlayer::iter() {
                    // We cannot spawn the client on startup. We spawn them later with the input manager.
                    if stdb_player.client_id == spacetimedb_sdk::identity::identity().unwrap() {
                        continue;
                    };

                    c.spawn(PlayerBundle::new(Player {
                        data: stdb_player.clone(),
                    }));
                }
            }
            UncbMessage::Disconnected => {
                break;
            }
            _ => {}
        }
    }
}

/// Listens for the `UncbMessage::PlayerInserted` message and spawns all players recently inserted
/// using a `PlayerBundle`. If the player received in the message has the same `client_id` as the
/// current client, it adds an input manager onto the player, since it's the client.
fn refresh_players(mut c: Commands, mut er: EventReader<UncbEvent>) {
    let mut spawnable_players: Vec<StdbPlayer> = Vec::new();

    for ev in er.read() {
        match &ev.message {
            UncbMessage::PlayerInserted { data, event } => {
                spawnable_players.push(data.clone());
            }
            _ => {}
        }
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

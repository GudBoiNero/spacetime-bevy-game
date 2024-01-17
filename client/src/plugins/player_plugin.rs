use bevy::{
    a11y::accesskit::Action,
    app::{App, Plugin, Startup, Update},
    ecs::{
        query::With,
        system::{Commands, Query},
    },
    input::keyboard::KeyCode,
    log::info,
};
use leafwing_input_manager::{action_state::ActionState, input_map::InputMap, InputManagerBundle};
use spacetimedb_sdk::table::TableType;

use crate::{
    components::player::{Player, PlayerBundle},
    create_player, identity_leading_hex,
    util::actions::GameActions,
    StdbPlayer,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (create_player))
            .add_systems(Update, (update_players));
    }
}

/// Called every frame.
fn update_players(mut c: Commands, q: Query<&Player>) {
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

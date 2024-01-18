use bevy::{
    a11y::accesskit::Action,
    app::{App, Plugin, Startup, Update},
    ecs::{
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
    components::player::{Player, PlayerBundle},
    identity_leading_hex,
    resources::uncb_receiver::{UncbEvent, UncbMessage, UncbReceiver},
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
        app.add_systems(Startup, (refresh_players))
            .add_systems(Update, (player_movement, refresh_players, update_players));
    }
}

/// Finds all currently spawned `Player`s and all `StdbPlayer`s within the database. \
/// Spawns only the `StdbPlayer`s that do not have a spawned `Player` with a corresponding `Identity`. \
/// Adds an `InputManagerBundle::<GameActions>` bundle to the *local* `Player` bundle.
fn refresh_players(mut c: Commands, q: Query<&Player>, er: EventReader<UncbEvent>) {}

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
}

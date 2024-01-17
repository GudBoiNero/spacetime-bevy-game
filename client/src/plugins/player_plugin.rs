use bevy::{
    a11y::accesskit::Action,
    app::{App, Plugin, Startup, Update},
    ecs::{
        query::With,
        system::{Commands, Query},
    },
};
use spacetimedb_sdk::table::TableType;

use crate::{
    components::player::{Player, PlayerBundle},
    create_player, identity_leading_hex, StdbPlayer,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_player)
            .add_systems(Update, (update_players));
    }
}

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
        println!("Spawned player: {}", identity_leading_hex(&spawn.client_id));
        c.spawn(PlayerBundle::new(Player { data: spawn }));
    }
}

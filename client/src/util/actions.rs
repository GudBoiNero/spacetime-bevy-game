use bevy::{log::info, math::Vec2, reflect::Reflect};
use leafwing_input_manager::{action_state::ActionState, Actionlike};

use super::bool_to_f32;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum GameActions {
    W,
    A,
    S,
    D,
}

pub fn get_input_vector(action_state: &ActionState<GameActions>) -> Vec2 {
    Vec2 {
        x: bool_to_f32(action_state.pressed(GameActions::D))
            - bool_to_f32(action_state.pressed(GameActions::A)),

        y: bool_to_f32(action_state.pressed(GameActions::W))
            - bool_to_f32(action_state.pressed(GameActions::S)),
    }
}

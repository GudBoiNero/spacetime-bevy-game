use bevy::prelude::*;
pub struct PlayerSystem;

impl PlayerSystem {
    
    pub fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::update_movement);
    }
    fn update_movement() {

    }
}
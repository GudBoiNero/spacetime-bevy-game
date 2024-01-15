use bevy::prelude::Component;
use spacetimedb_sdk::identity::Identity;

#[derive(Component)]
pub struct Owner {
    pub id: Identity
}

impl Owner {
    fn is_owner(&self) -> bool {
        spacetimedb_sdk::identity::identity().expect("Could not get Identity.") == self.id
    }
}
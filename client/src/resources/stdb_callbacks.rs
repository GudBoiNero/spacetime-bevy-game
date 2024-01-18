use std::any::Any;

use bevy::{
    app::App,
    ecs::{schedule::IntoSystemConfigs, system::Resource},
};
use spacetimedb_sdk::{
    reducer::Reducer,
    spacetimedb_lib::{de::DeserializeOwned, ser::Serialize},
    table::{InsertCallbackId, TableType},
};

#[derive(Resource)]
pub struct StdbCallbacks {}

impl StdbCallbacks {
    pub fn new(&self, app: &mut App) -> Self {
        Self {}
    }

    pub fn add_callback<M, T: TableType>(
        &mut self,
        event: impl FnMut(&T, Option<&T::ReducerEvent> + Send + 'static) -> InsertCallbackId<T>,
        callbacks: (impl IntoSystemConfigs<M>,),
    ) -> &mut Self {
        self
    }
}

use bevy::prelude::*;

fn main() {
    register_callbacks();
    connect_to_db();
    subscribe_to_tables();
    user_input_loop();

    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, init)
        .add_systems(Update, update)
        .run();
}


fn init() {

}

fn update() {

}

fn register_callbacks() {

}

fn connect_to_db() {

}

fn subscribe_to_tables() {

}

fn user_input_loop() {
    
}
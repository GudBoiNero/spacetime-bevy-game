use spacetimedb::{spacetimedb, Identity, SpacetimeType, spacetimedb_lib::primary_key, ReducerContext, Result};

#[spacetimedb(table)]
#[derive(Clone)]
pub struct Client {
    #[primarykey]
    pub client_id: Identity,
    pub connected: bool
}

#[derive(SpacetimeType, Clone)]
pub struct StdbVector2 {
    pub x: f32,
    pub y: f32,
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct Object {
    #[primarykey]
    #[autoinc]
    pub object_id: u64,
    pub position: StdbVector2,
    pub velocity: StdbVector2,
}

#[spacetimedb(table)]
#[derive(Clone)]
pub struct Player {
    #[primarykey]
    pub object_id: u64,

    #[unique]
    pub client_id: Identity,
}

#[spacetimedb(init)]
pub fn init() {
    // Called when the module is initially published
}



// Called when the client connects, we update the logged_in state to true
#[spacetimedb(connect)]
pub fn client_connected(ctx: ReducerContext) {
    // called when the client connects, we update the logged_in state to true
    update_client_login_state(ctx, true);
}


// Called when the client disconnects, we update the logged_in state to false
#[spacetimedb(disconnect)]
pub fn client_disconnected(ctx: ReducerContext) {
    // Called when the client disconnects, we update the logged_in state to false
    update_client_login_state(ctx, false);
}

// This helper function gets the PlayerComponent, sets the logged
// in variable and updates the PlayerComponent table row.
pub fn update_client_login_state(ctx: ReducerContext, connected: bool) {
    if let Some(client) = Client::filter_by_client_id(&ctx.sender) {
        // We clone the PlayerComponent so we can edit it and pass it back.
        let mut client: Client = client.clone();
        client.connected = connected;
        Client::filter_by_client_id(&&client.client_id.clone());
    }
}

// This reducer is called when the user logs in for the first time and
// enters a username
#[spacetimedb(reducer)]
pub fn create_player(ctx: ReducerContext) -> Result<(), String> {
    // Get the Identity of the client who called this reducer
    let client_id = ctx.sender;

    // Make sure we don't already have a player with this identity
    if Player::filter_by_client_id(&client_id).is_some() {
        log::info!("Player already exists");
        return Err("Player already exists".to_string());
    }

    // Create a new entity for this player and get a unique `entity_id`.
    let object_id = Object::insert(Object 
    { 
        object_id: 0, 
        position: StdbVector2 { x: 0.0, y: 0.0 },
        velocity: StdbVector2 { x: 0.0, y: 0.0 },
    }).expect("Failed to create a unique PlayerComponent.").object_id;

    // The PlayerComponent uses the same entity_id and stores the identity of
    // the owner, username, and whether or not they are logged in.
    Player::insert(Player {
        object_id,
        client_id
    }).expect("Failed to insert player component.");

    log::info!("Player created: {}", object_id);

    Ok(())
}

#[spacetimedb(reducer)]
pub fn update_player_pos(ctx: ReducerContext, position: StdbVector2, velocity: StdbVector2) -> Result<(), String> {
    if let Some(player) = Player::filter_by_client_id(&ctx.sender) {
        if let Some(mut object) = Object::filter_by_object_id(&player.object_id) {
            object.position = position;
            object.velocity = velocity;
            Object::update_by_object_id(&player.object_id, object);
            return Ok(());
        }
    }

    return Err("Player not found".to_string());
}
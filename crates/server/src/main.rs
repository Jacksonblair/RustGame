use std::collections::HashMap;

use bevy::{
    app::{App, Startup, Update},
    ecs::{component::Component, entity::Entity, system::Resource},
    DefaultPlugins,
};
use bevy_renet::{renet::ClientId, transport::NetcodeServerPlugin, RenetServerPlugin};
use serde::{Deserialize, Serialize};
use systems::{
    handle_events_system::handle_events_system, receive_message_system::receive_message_system,
    send_message_system::send_message_system, server_setup::server_setup,
};
mod systems;

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<ClientId, Entity>,
}

#[derive(Debug, Component)]
struct Player {
    id: ClientId,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    PlayerCreate {
        entity: Entity,
        id: ClientId,
        translation: [f32; 3],
    },
    PlayerRemove {
        id: ClientId,
    },
    // SpawnProjectile {
    //     entity: Entity,
    //     translation: [f32; 3],
    // },
    // DespawnProjectile {
    //     entity: Entity,
    // },
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_plugins(RenetServerPlugin);
    app.add_plugins(NetcodeServerPlugin);
    app.add_systems(Startup, server_setup);
    app.add_systems(
        Update,
        (
            send_message_system,
            receive_message_system,
            handle_events_system,
        ),
    );
    app.insert_resource(ServerLobby::default());

    app.run();
}

use std::collections::HashMap;

use add_input::add_input;
use add_netcode_network::add_netcode_network;
use bevy::{
    app::{App, Startup, Update},
    asset::AssetServer,
    core_pipeline::core_2d::Camera2dBundle,
    ecs::{
        entity::Entity,
        event::EventReader,
        schedule::{IntoSystemConfigs, IntoSystemSetConfigs, SystemSet},
        system::{Commands, Res, ResMut, Resource},
    },
    sprite::SpriteBundle,
    transform::components::Transform,
    utils::default,
    DefaultPlugins,
};
use bevy_renet::{
    client_connected,
    renet::{ClientId, RenetClient},
    RenetClientPlugin,
};
use shared::{connection_config, ClientChannel, PlayerCommands, ServerChannel, ServerMessages};
use systems::{connection_system, send_message_system};

mod add_input;
mod add_netcode_network;
mod components;
mod resources;
mod systems;

#[derive(Debug, Resource)]
struct CurrentClientId(u64);

/** Map client entity to server entity */
#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

/** Map client entity to server entity */
#[derive(Default, Resource)]
struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug, Default, Resource)]
struct ClientLobby {
    players: HashMap<ClientId, PlayerInfo>,
}

/**
 * Client lobby tracks players in scene
 * NetworkMapping is a hashmap of entities
 * -- client send input
 *
 * client send player commands
 * client sync players
 *
 *
 * I map input to events. Or should i rename them as commands.
 *
 *
 * Reliable Channels
 * - Chat ?
 * - Entities
 *
 * Unreliable channels
 * - Discrete events
 *      - Not stored.
 *
 * Detect input
 * - Parse input into commands
 * - Send commands to server
 *
 * vs
 *
 * Detect input
 * - Set input resource state based on input
 * - Parse into into commands
 * - Send input to server
 * - Send commands to server
 *
 *
 */

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connected;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(bevy_renet::transport::NetcodeClientPlugin);

    // Configure a system
    // app.configure_sets(Update, Connected.run_if(client_connected()));

    add_netcode_network(&mut app);
    add_input(&mut app);

    // app.add_systems(Update, (player_input, camera_follow, update_target_system));
    app.add_systems(
        Update,
        (
            // client_send_input,
            // client_send_player_commands,
            client_sync_players,
            send_message_system,
        )
            .run_if(client_connected()),
    );
    // app.insert_resource(RenetClientVisualizer::<200>::new(RenetVisualizerStyle::default()));
    // app.add_systems(Update, update_visulizer_system);

    app.add_event::<PlayerCommands>();
    app.add_systems(Update, (connection_system));

    let client = RenetClient::new(connection_config());
    app.insert_resource(client);
    app.insert_resource(ClientLobby::default());
    app.insert_resource(NetworkMapping::default());

    app.world.spawn((Camera2dBundle {
        transform: Transform::from_xyz(100.0, 200.0, 0.0),
        ..default()
    },));

    // app.add_systems(Startup, (setup_level, setup_camera, setup_target));

    app.run();
}

fn client_sync_players(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    client_id: Res<CurrentClientId>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkMapping>,
    asset_server: Res<AssetServer>,
) {
    let client_id = client_id.0;
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        println!("{:?}", server_message);
        match server_message {
            /*
             * PlayerCreate msg includs an id, translation, and an entity
             * We spawn a Pbrbundle using the traslation deets
             *
             * Attach controlledPLayer marker comp
             * Create a player info struct using Entity (which is just an index)
             * And the client_entity
             *
             * Then we chuck that struct into the lobby
             *      (For Keeping track of players in game)
             * And then map the server entity to the client entity in network mapping
             *      (For mapping server entity IDs to client entity IDs)
             *      (So we can sync up)
             */
            ServerMessages::PlayerCreate {
                id,
                translation,
                entity,
            } => {
                println!("Player {} connected.", id);
                let client_entity = commands.spawn(SpriteBundle {
                    texture: asset_server.load("test.png"),
                    transform: Transform::from_xyz(translation[0], translation[1], translation[2]),
                    ..default()
                });

                // if client_id == id.raw() {
                //     client_entity.insert(ControlledPlayer);
                // }

                let player_info = PlayerInfo {
                    server_entity: entity,
                    client_entity: client_entity.id(),
                };

                lobby.players.insert(id, player_info);
                network_mapping.0.insert(entity, client_entity.id());
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player {} disconnected.", id);
                if let Some(PlayerInfo {
                    server_entity,
                    client_entity,
                }) = lobby.players.remove(&id)
                {
                    commands.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
            ServerMessages::SpawnProjectile {
                entity,
                translation,
            } => {
                // let projectile_entity = commands.spawn(PbrBundle {
                //     mesh: meshes.add(
                //         Mesh::try_from(Icosphere {
                //             radius: 0.1,
                //             subdivisions: 5,
                //         })
                //         .unwrap(),
                //     ),
                //     material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
                //     transform: Transform::from_translation(translation.into()),
                //     ..Default::default()
                // });
                // network_mapping.0.insert(entity, projectile_entity.id());
            }
            ServerMessages::DespawnProjectile { entity } => {
                // if let Some(entity) = network_mapping.0.remove(&entity) {
                //     commands.entity(entity).despawn();
                // }
            }
        }
    }

    while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
        // let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();

        // for i in 0..networked_entities.entities.len() {
        //     if let Some(entity) = network_mapping.0.get(&networked_entities.entities[i]) {
        //         let translation = networked_entities.translations[i].into();
        //         let transform = Transform {
        //             translation,
        //             ..Default::default()
        //         };
        //         commands.entity(*entity).insert(transform);
        //     }
        // }
    }
}

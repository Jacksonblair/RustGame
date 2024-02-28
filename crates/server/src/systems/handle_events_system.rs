use bevy::{
    ecs::{
        entity::Entity,
        event::EventReader,
        system::{Commands, Query, ResMut},
    },
    pbr::PbrBundle,
    sprite::SpriteBundle,
    transform::components::Transform,
};
use bevy_renet::renet::{RenetServer, ServerEvent};
use shared::{ClientChannel, PlayerCommands, ServerChannel, ServerMessages};

use crate::{Player, ServerLobby};

pub fn handle_events_system(
    mut server_events: EventReader<ServerEvent>,
    mut commands: Commands,
    mut lobby: ResMut<ServerLobby>,
    players: Query<(Entity, &Player, &Transform)>,
    mut server: ResMut<RenetServer>,
) {
    for event in server_events.read() {
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Client {client_id} connected");

                // Initialize other players for this new client
                for (entity, player, transform) in players.iter() {
                    let translation: [f32; 3] = transform.translation.into();
                    let message = bincode::serialize(&ServerMessages::PlayerCreate {
                        id: player.id,
                        entity,
                        translation,
                    })
                    .unwrap();
                    server.send_message(*client_id, ServerChannel::ServerMessages, message);
                }

                // Spawn new player
                let transform = Transform::from_xyz(0., 0., 0.);
                let player_entity = commands
                    .spawn(SpriteBundle {
                        transform,
                        ..Default::default()
                    })
                    .insert(Player { id: *client_id })
                    .id();

                lobby.players.insert(*client_id, player_entity);

                let translation: [f32; 3] = transform.translation.into();
                let message = bincode::serialize(&ServerMessages::PlayerCreate {
                    id: *client_id,
                    entity: player_entity,
                    translation,
                })
                .unwrap();

                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Client {client_id} disconnected: {reason}");

                if let Some(player_entity) = lobby.players.remove(client_id) {
                    commands.entity(player_entity).despawn();
                }
                let message =
                    bincode::serialize(&ServerMessages::PlayerRemove { id: *client_id }).unwrap();

                server.broadcast_message(ServerChannel::ServerMessages, message);
            }
        }

        for client_id in server.clients_id() {
            while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
                let command: PlayerCommands = bincode::deserialize(&message).unwrap();
                println!("{:?}", command);
                // match command {
                // PlayerCommands::BasicAttack { mut cast_at } => {
                //     println!(
                //         "Received basic attack from client {}: {:?}",
                //         client_id, cast_at
                //     );

                //     if let Some(player_entity) = lobby.players.get(&client_id) {
                //         if let Ok((_, _, player_transform)) = players.get(*player_entity) {
                //             cast_at[1] = player_transform.translation[1];

                //             let direction =
                //                 (cast_at - player_transform.translation).normalize_or_zero();
                //             let mut translation =
                //                 player_transform.translation + (direction * 0.7);
                //             translation[1] = 1.0;

                //             let fireball_entity = spawn_fireball(
                //                 &mut commands,
                //                 &mut meshes,
                //                 &mut materials,
                //                 translation,
                //                 direction,
                //             );
                //             let message = ServerMessages::SpawnProjectile {
                //                 entity: fireball_entity,
                //                 translation: translation.into(),
                //             };
                //             let message = bincode::serialize(&message).unwrap();
                //             server.broadcast_message(ServerChannel::ServerMessages, message);
                //         }
                //     }
                // }
            }
        }
    }
}

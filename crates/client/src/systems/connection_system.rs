use std::{net::UdpSocket, time::SystemTime};

use bevy::ecs::{
    event::EventReader,
    system::{Commands, ResMut},
};
use bevy_renet::renet::{
    transport::{ClientAuthentication, NetcodeClientTransport},
    ConnectionConfig, RenetClient,
};
use shared::{PlayerCommands, PROTOCOL_ID, SERVER_ADDR};

use crate::CurrentClientId;

pub fn connection_system(
    mut client: Option<ResMut<RenetClient>>,
    mut commands: Commands,
    mut eventreader: EventReader<PlayerCommands>,
) {
    for e in eventreader.read() {
        println!("EVENT: {:?}", e);
        match e {
            PlayerCommands::DisconnectFromServer => {
                if client.is_some() && client.as_mut().unwrap().is_connected() {
                    println!("Disconnecting...");
                    client.as_mut().unwrap().disconnect();
                    commands.remove_resource::<RenetClient>();
                    commands.remove_resource::<NetcodeClientTransport>();
                }
            }
            PlayerCommands::ConnectToServer => {
                if client.is_some() && client.as_mut().unwrap().is_connected() {
                    println!("AREADY CONNECTED");
                    return;
                }

                let client = RenetClient::new(ConnectionConfig::default());
                commands.insert_resource(client);

                let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

                let server_addr = SERVER_ADDR.parse().unwrap();
                let current_time = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap();
                let client_id = current_time.as_millis() as u64;

                let authentication = ClientAuthentication::Unsecure {
                    client_id,
                    protocol_id: PROTOCOL_ID,
                    server_addr,
                    user_data: None,
                };

                let transport =
                    NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
                commands.insert_resource(transport);
                commands.insert_resource(CurrentClientId(client_id));
            }
            _ => (),
        }
    }
}

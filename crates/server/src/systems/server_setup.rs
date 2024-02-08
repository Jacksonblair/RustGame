use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket},
    process::Command,
    time::SystemTime,
};

use bevy::ecs::system::Commands;
use bevy_renet::renet::{
    transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig},
    ConnectionConfig, RenetServer,
};

const SERVER_ADDR: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 1234);

pub fn server_setup(mut commands: Commands) {
    let server = RenetServer::new(ConnectionConfig::default());
    commands.insert_resource(server);

    let socket = UdpSocket::bind(SERVER_ADDR).unwrap();
    let server_config = ServerConfig {
        current_time: SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        max_clients: 64,
        protocol_id: 0,
        authentication: ServerAuthentication::Unsecure,
        public_addresses: vec![SERVER_ADDR],
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    commands.insert_resource(transport);
}

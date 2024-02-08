use bevy::{
    app::{App, Update},
    ecs::{event::EventReader, system::Resource},
};
use bevy_renet::renet::transport::{
    ClientAuthentication, NetcodeClientTransport, NetcodeTransportError,
};
use bevy_renet::renet::{ConnectionConfig, RenetClient};
use shared::{PROTOCOL_ID, SERVER_ADDR};
use std::{net::UdpSocket, time::SystemTime};

// ref
// https://github.com/lucaspoffo/renet/blob/master/demo_bevy/src/bin/client.rs#L148

#[derive(Debug, Resource)]
struct CurrentClientId(u64);

pub fn add_netcode_network(app: &mut App) {
    app.add_plugins(bevy_renet::transport::NetcodeClientPlugin);

    // WHAT DO WITH THIS?
    // app.configure_sets(Update, Connected.run_if(client_connected()));

    let client = RenetClient::new(ConnectionConfig::default());
    let server_addr = SERVER_ADDR.parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();

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

    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.insert_resource(client);
    app.insert_resource(transport);
    app.insert_resource(CurrentClientId(client_id));

    // If any error is found we just panic
    // #[allow(clippy::never_loop)]
    // fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
    //     for e in renet_error.read() {
    //         panic!("{}", e);
    //     }
    // }

    // app.add_systems(Update, panic_on_error_system);
}

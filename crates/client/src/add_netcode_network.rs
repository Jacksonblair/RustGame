use bevy::app::App;
use bevy::app::Update;
use bevy::ecs::event::EventReader;
use bevy::ecs::system::Commands;
use bevy::ecs::system::ResMut;
use bevy_renet::renet::transport::NetcodeClientTransport;
use bevy_renet::renet::transport::NetcodeTransportError;
use bevy_renet::renet::{ConnectionConfig, RenetClient};

use crate::CurrentClientId;

pub fn add_netcode_network(app: &mut App) {
    // If any error is found we just panic
    fn panic_on_error_system(
        mut renet_error: EventReader<NetcodeTransportError>,
        mut transport: Option<ResMut<NetcodeClientTransport>>,
        mut commands: Commands,
    ) {
        for e in renet_error.read() {
            match e {
                NetcodeTransportError::IO(err) => {
                    println!("{}", e);
                    println!("Disconnecting and removing resources");
                    transport.as_mut().unwrap().disconnect();
                    commands.remove_resource::<RenetClient>();
                    commands.remove_resource::<NetcodeClientTransport>();
                    commands.remove_resource::<CurrentClientId>();
                }
                NetcodeTransportError::Netcode(err) => {
                    println!("{}", e);
                }
                NetcodeTransportError::Renet(err) => {
                    println!("{}", e);
                }
            }
            // panic!("{}", e);
        }
    }

    app.add_systems(Update, panic_on_error_system);
}

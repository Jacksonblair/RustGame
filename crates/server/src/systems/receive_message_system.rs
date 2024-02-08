use bevy::ecs::system::ResMut;
use bevy_renet::renet::{DefaultChannel, RenetServer};

pub fn receive_message_system(mut server: ResMut<RenetServer>) {
    // Receive message from all clients
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            // Handle received message
            println!("{:?}", message);
        }
    }
}

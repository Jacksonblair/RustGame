use bevy::ecs::system::ResMut;
use bevy_renet::renet::{DefaultChannel, RenetServer};

pub fn send_message_system(mut server: ResMut<RenetServer>) {
    let channel_id = 0;
    // Send a text message for all clients
    // The enum DefaultChannel describe the channels used by the default configuration
    server.broadcast_message(DefaultChannel::ReliableOrdered, "server message");
}

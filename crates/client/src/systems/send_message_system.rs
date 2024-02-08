use bevy::ecs::{event::EventReader, system::ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use shared::GameEvents;

pub fn send_message_system(mut client: ResMut<RenetClient>, mut events: EventReader<GameEvents>) {
    for e in events.read() {
        match e {
            GameEvents::MoveBack => {
                println!("SENDING MESSAGE");
                // Send a text message to the server
                client.send_message(DefaultChannel::ReliableOrdered, "server message");
            }
            _ => (),
        }
    }
}

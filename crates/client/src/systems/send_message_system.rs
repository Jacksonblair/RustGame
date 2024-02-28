use bevy::ecs::{event::EventReader, system::ResMut};
use bevy_renet::renet::RenetClient;
use shared::{ClientChannel, PlayerCommands};

pub fn send_message_system(
    mut client: ResMut<RenetClient>,
    mut events: EventReader<PlayerCommands>,
) {
    for e in events.read() {
        println!("MSG: {:?}", e);
        match e {
            PlayerCommands::MoveBack => {
                println!("SENDING MESSAGE");
                // Send a text message to the server
                client.send_message(ClientChannel::Command, "server message");
            }
            _ => (),
        }
    }
}

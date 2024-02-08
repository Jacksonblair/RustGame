use bevy::ecs::{event::EventReader, system::ResMut};
use bevy_renet::renet::RenetClient;
use shared::GameEvents;

pub fn connection_system(client: ResMut<RenetClient>, mut eventreader: EventReader<GameEvents>) {
    for e in eventreader.read() {
        match e {
            GameEvents::ConnectToServer => {
                if client.is_disconnected() {
                    println!("SHOULD CONNECT")
                } else {
                    println!("AREADY CONNECTED")
                };
            }
            _ => (),
        }
    }
}

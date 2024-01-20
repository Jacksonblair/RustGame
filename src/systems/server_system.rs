use bevy::ecs::system::ResMut;
use netcode::Server;
use std::{
    thread,
    time::{Duration, Instant},
};

use crate::resources::packets::Packets;

pub fn server_system(mut packets: ResMut<Packets>) {
    // SETUP SERVER RESOURCES

    // Create a server resource
    let protocol_id = 0x11223344u64; // a unique number that must match the client's protocol id
    let private_key = netcode::generate_key(); // you can also provide your own key
    let mut server = Server::new("0.0.0.0:5555", protocol_id, private_key).unwrap();

    // Run the server at 60Hz
    let start = Instant::now();
    let tick_rate = Duration::from_secs_f64(1.0 / 60.0);
    loop {
        let elapsed = start.elapsed().as_secs_f64();
        server.update(elapsed);
        while let Some((data, from)) = server.recv() {
            packets.addPacket(data, from.0);
        }
        thread::sleep(tick_rate);
    }
}

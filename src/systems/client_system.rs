use bevy::ecs::system::Commands;
use std::{
    thread,
    time::{Duration, Instant},
};

pub fn client_system(mut commands: Commands) {
    // Generate a connection token for the client
    // let protocol_id = 0x11223344u64; // a unique number that must match the server's protocol id
    // let private_key = netcode::generate_key(); // you can also provide your own key
    // let client_id = 123u64; // globally unique identifier for an authenticated client
    // let server_address = "my-domain.com:5555"; // the server's public address (can also be multiple addresses)
    // let connect_token = ConnectToken::build(server_address, protocol_id, client_id, private_key)
    //     .generate()
    //     .unwrap();

    // // Start the client
    // let token_bytes = connect_token.try_into_bytes().unwrap();
    // let mut client = Client::new(&token_bytes).unwrap();
    // client.connect();

    // // Run the client at 60Hz
    // let start = Instant::now();
    // let tick_rate = Duration::from_secs_f64(1.0 / 60.0);
    // loop {
    //     let elapsed = start.elapsed().as_secs_f64();
    //     client.update(elapsed);
    //     if let Some(packet) = client.recv() {
    //         // ...
    //     }
    //     thread::sleep(tick_rate);
    // }
}

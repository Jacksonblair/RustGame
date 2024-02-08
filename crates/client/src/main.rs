use add_netcode_network::add_netcode_network;
use bevy::{
    app::{App, Startup, Update},
    DefaultPlugins,
};
use bevy_renet::RenetClientPlugin;
use shared::GameEvents;
use systems::{
    client_setup::client_setup, connection_system::connection_system, input_system::input_system,
    send_message_system::send_message_system,
};

mod add_netcode_network;
mod components;
mod resources;
mod systems;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(RenetClientPlugin);
    app.add_event::<GameEvents>();

    add_netcode_network(&mut app);

    app.add_systems(Startup, client_setup);
    app.add_systems(
        Update,
        (input_system, send_message_system, connection_system),
    );
}

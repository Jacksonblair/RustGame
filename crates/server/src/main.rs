use bevy::{
    app::{App, Startup, Update},
    DefaultPlugins,
};
use bevy_renet::{transport::NetcodeServerPlugin, RenetServerPlugin};
use systems::{
    handle_events_system::handle_events_system, receive_message_system::receive_message_system,
    send_message_system::send_message_system, server_setup::server_setup,
};
mod systems;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RenetServerPlugin)
        .add_plugins(NetcodeServerPlugin)
        .add_systems(Startup, server_setup)
        .add_systems(
            Update,
            (
                send_message_system,
                receive_message_system,
                handle_events_system,
            ),
        )
        .run();
}

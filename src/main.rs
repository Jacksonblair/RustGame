use arg_parser::{ArgParser, DefaultArgParser};
use bevy::prelude::*;
use events::GameEvents;
use resources::packets::Packets;
use systems::{
    client_setup::client_setup, client_system::client_system, input_system::input_system,
    server_system::server_system,
};

mod arg_parser;
mod components;
mod entities;
mod events;
mod resources;
mod systems;

fn run_if_client(parser: &dyn ArgParser) -> impl Fn() -> bool {
    let is_client = parser.is_client();
    return move || is_client;
}

fn run_if_server(parser: &dyn ArgParser) -> impl Fn() -> bool {
    let is_server = parser.is_server();
    return move || is_server;
}

fn main() {
    let arg_parser = DefaultArgParser::new(std::env::args().collect());
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Packets { packets: vec![] })
        .add_event::<GameEvents>()
        .add_systems(Update, (server_system).run_if(run_if_server(&arg_parser)))
        .add_systems(Startup, (client_setup).run_if(run_if_client(&arg_parser)))
        .add_systems(
            Update,
            (client_system, input_system).run_if(run_if_client(&arg_parser)),
        )
        // .add_systems(Startup, server_system)
        .run();
}

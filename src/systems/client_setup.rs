use bevy::ecs::system::Commands;

use crate::components::input_context::{
    GameInputContextHandler, InputContext, MenuInputContextHandler,
};

pub fn client_setup(mut commands: Commands) {
    // Add input contexts
    let mut c1 = InputContext::new(GameInputContextHandler::new(), String::from("gameplay"));
    c1.activate();

    commands.spawn(c1);
    commands.spawn(InputContext::new(
        MenuInputContextHandler::new(),
        String::from("menu"),
    ));
}

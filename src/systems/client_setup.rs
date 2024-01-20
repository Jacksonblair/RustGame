use bevy::ecs::system::Commands;

use crate::components::input_context::{
    GameInputContextHandler, InputContext, SecondaryInputContextHandler,
};

pub fn client_setup(mut commands: Commands) {
    let mut c = InputContext::new(GameInputContextHandler::new());
    c.activate();

    // Add input contexts
    commands.spawn(c);
    commands.spawn(InputContext::new(SecondaryInputContextHandler::new()));
}

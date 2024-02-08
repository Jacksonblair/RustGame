use crate::{
    components::input_context::{GameInputContextHandler, InputContext, MenuInputContextHandler},
    resources::input_contexts::InputContexts,
};
use bevy::ecs::system::Commands;

pub fn client_setup(mut commands: Commands) {
    // Add input contexts
    let input_context1 =
        InputContext::new(GameInputContextHandler::new(), String::from("gameplay"));
    let input_context2 = InputContext::new(MenuInputContextHandler::new(), String::from("menu"));
    let contexts = InputContexts {
        contexts: vec![input_context1, input_context2],
    };
    commands.insert_resource(contexts);
}

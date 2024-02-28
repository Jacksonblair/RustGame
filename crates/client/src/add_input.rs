use crate::{
    components::input_context::{GameInputContextHandler, InputContext, MenuInputContextHandler},
    resources::input_contexts::InputContexts,
    systems::input_system,
};
use bevy::app::{App, Update};

pub fn add_input(app: &mut App) {
    // Add input contexts
    let input_context1 =
        InputContext::new(GameInputContextHandler::new(), String::from("gameplay"));
    let input_context2 = InputContext::new(MenuInputContextHandler::new(), String::from("menu"));
    let contexts = InputContexts {
        contexts: vec![input_context1, input_context2],
    };

    app.insert_resource(contexts);
    app.add_systems(Update, input_system);
}

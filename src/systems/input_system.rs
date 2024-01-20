use crate::{components::input_context::InputContext, events::GameEvents};
use bevy::{
    ecs::{
        event::{EventReader, EventWriter},
        system::Query,
    },
    input::{keyboard::KeyboardInput, mouse::MouseButtonInput},
};

pub fn input_system(
    mut input_contexts: Query<&InputContext>,
    mut er_keyboard: EventReader<KeyboardInput>,
    mut er_mouse: EventReader<MouseButtonInput>,
    mut ew: EventWriter<GameEvents>,
) {
    let mut generated_events: Vec<GameEvents> = vec![];
    let mut keyboard_events: Vec<&KeyboardInput> = er_keyboard.read().into_iter().collect();
    let mouse_events: Vec<&MouseButtonInput> = er_mouse.read().into_iter().collect();

    // TODO: Somehow order input contexts.
    if keyboard_events.len() == 0 && mouse_events.len() == 0 {
        return;
    }

    for inputcontext in input_contexts.iter() {
        // Only run active input contexts
        if inputcontext.is_active() == false {
            return;
        }
        let mut result = inputcontext.handler.handle_input(&mut keyboard_events);
        generated_events.append(&mut result.generated_events);
    }

    // Dispatch generated events
    ew.send_batch(generated_events);
}

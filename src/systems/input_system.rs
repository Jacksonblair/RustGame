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
    let mut keyboard_events: Vec<&KeyboardInput> = er_keyboard.read().collect();
    // let mouse_events: Vec<&MouseButtonInput> = er_mouse.read().into_iter().collect();

    if keyboard_events.len() == 0 {
        return;
    }

    // Run input through all active input contexts
    for inputcontext in input_contexts.iter() {
        // if inputcontext.is_active() == false {
        //     continue;
        // }
        let mut result = inputcontext.handle_input(&mut keyboard_events);
        generated_events.append(&mut result.generated_events);
    }

    if (generated_events.len() > 0) {
        println!("{:?}", generated_events);
    }

    // Dispatch generated events
    ew.send_batch(generated_events);
}

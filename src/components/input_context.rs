use std::fs;

use crate::events::GameEvents;
use bevy::{
    ecs::component::Component,
    input::keyboard::{KeyCode, KeyboardInput},
    utils::hashbrown::HashMap,
};
use toml::{map::Map, *};

type KeyboardInputMap = HashMap<KeyCode, GameEvents>;

fn create_input_map(config: Map<String, Value>) -> KeyboardInputMap {
    let mut input_map = HashMap::<KeyCode, GameEvents>::new();

    for (key, val) in config.iter() {
        if val.is_str() == false {
            continue;
        }
        let event_str = val.as_str().unwrap();

        let keycode: Result<KeyCode, ()>;
        let event: Result<GameEvents, ()>;

        match key.as_str() {
            "W" => keycode = Ok(KeyCode::W),
            "A" => keycode = Ok(KeyCode::A),
            "S" => keycode = Ok(KeyCode::S),
            "D" => keycode = Ok(KeyCode::D),
            _ => {
                println!("Could not map ({:?}) to Keycode", key);
                continue;
            }
        }

        match event_str {
            "MoveFwd" => event = Ok(GameEvents::MoveForward),
            "MoveBack" => event = Ok(GameEvents::MoveDown),
            "MoveLeft" => event = Ok(GameEvents::MoveLeft),
            "MoveRight" => event = Ok(GameEvents::MoveRight),
            _ => {
                println!("Could not map ({:?}) to event", event_str);
                continue;
            }
        }

        if keycode.is_ok() == false || event.is_ok() == false {
            continue;
        }

        input_map.insert(keycode.unwrap(), event.unwrap());
    }

    input_map
}

/**
 * I want to map keycode actions
 * I want to map mouseevent to actions
 * I want to map touchevent to actions
 * ...etc
 *
 */

// rawinputconstants
// inputconstants
// input map (maps raw inputs to input constants)
/**
 * Read input map from yaml
 * Match raw input against input map, to find inputconstant
 *
 * So i press "A", context consumes A, matches it to input based on inputmap
 * - A == Events::MoveFwd
 * The game context check if A is mapped to any of its inputs
 * If it is, it fires off an event.
 *
 * So what i need to do is dynamically check the input against a inputmap inside the context
 */

// struct InputMapper {}
// impl InputMapper {
//     fn map_input_str_to_event(str: String) -> Event {
//         match(str) {
//             "W"
//         }
//     }
// }

struct Config {
    input_map: HashMap<KeyCode, GameEvents>,
}

pub struct HandleInputResult {
    pub generated_events: Vec<GameEvents>,
}

pub trait InputContextHandler {
    fn handle_input(&self, input: &mut Vec<&KeyboardInput>) -> HandleInputResult;
}

#[derive(Component)]
pub struct InputContext {
    is_active: bool,
    keyboard_input_map: KeyboardInputMap,
    pub handler: Box<dyn InputContextHandler + Send + Sync>,
}

impl InputContext {
    pub fn is_active(&self) -> bool {
        return self.is_active;
    }
    pub fn activate(&mut self) {
        self.is_active = true;
    }
    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn new(handler: Box<dyn InputContextHandler + Send + Sync>) -> InputContext {
        let filename = "conf.toml";
        let contents = match fs::read_to_string(filename) {
            // If successful return the files text as `contents`.
            // `c` is a local variable.
            Ok(c) => c,
            // Handle the `error` case.
            Err(_) => {
                // Write `msg` to `stderr`.
                println!("Could not read file `{}`", filename);
                panic!();
            }
        };

        // Create input maps
        let input = contents.parse::<Table>().unwrap();
        let keyboard_input_map = create_input_map(input);
        // Todo: create other input maps

        return InputContext {
            is_active: false,
            keyboard_input_map,
            handler,
        };
    }
}

// -- Default InputContextHandler
pub struct GameInputContextHandler {}

impl GameInputContextHandler {
    pub fn new() -> Box<GameInputContextHandler> {
        return Box::new(GameInputContextHandler {});
    }
}

impl InputContextHandler for GameInputContextHandler {
    fn handle_input(&self, keyboard_events: &mut Vec<&KeyboardInput>) -> HandleInputResult {
        let mut events: Vec<GameEvents> = vec![];

        // Let event = map_input(input)
        // if event === MoveFwd
        // {  }

        // Consume events we care about
        keyboard_events.retain(|v| {
            match v.key_code {
                Some(code) => {
                    if code == KeyCode::W {
                        events.push(GameEvents::MoveForward);
                    }
                    if code == KeyCode::A {
                        events.push(GameEvents::MoveLeft);
                    }
                    if code == KeyCode::S {
                        events.push(GameEvents::MoveDown);
                    }
                    if code == KeyCode::D {
                        events.push(GameEvents::MoveRight);
                    }

                    return false;
                }
                None => {}
            }

            return true;
        });

        // Add some events.
        return HandleInputResult {
            generated_events: vec![],
        };
    }
}

// -- Secondary InputContextHandler
pub struct SecondaryInputContextHandler {}

impl SecondaryInputContextHandler {
    pub fn new() -> Box<SecondaryInputContextHandler> {
        return Box::new(SecondaryInputContextHandler {});
    }
}

// Default InputContextHandler
impl InputContextHandler for SecondaryInputContextHandler {
    fn handle_input(&self, keyboard_events: &mut Vec<&KeyboardInput>) -> HandleInputResult {
        // Consume events we care about
        keyboard_events.retain(|v| {
            match v.key_code {
                Some(code) => {
                    if code == KeyCode::A {
                        return false;
                    }
                }
                None => {}
            }

            return true;
        });

        println!("{:?}", keyboard_events);
        // do something.
        return HandleInputResult {
            generated_events: vec![],
        };
    }
}

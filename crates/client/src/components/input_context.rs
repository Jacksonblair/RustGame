use std::{collections::HashMap, fs};

use bevy::{
    ecs::component::Component,
    input::{
        keyboard::{KeyCode, KeyboardInput},
        ButtonState,
    },
};
use shared::PlayerCommands;
use toml::{map::Map, *};

type KeyboardInputMap = HashMap<KeyCode, PlayerCommands>;
type MappedKeyboardInput<'a> = Vec<(&'a KeyboardInput, PlayerCommands)>;

fn create_input_map(config: &Table) -> KeyboardInputMap {
    let mut input_map = HashMap::<KeyCode, PlayerCommands>::new();

    for (key, val) in config.iter() {
        if val.is_str() == false {
            continue;
        }

        let keycode: Result<KeyCode, ()>;
        let event: Result<PlayerCommands, ()>;

        match key.as_str() {
            "MoveFwd" => event = Ok(PlayerCommands::MoveForward),
            "MoveBack" => event = Ok(PlayerCommands::MoveBack),
            "MoveLeft" => event = Ok(PlayerCommands::MoveLeft),
            "MoveRight" => event = Ok(PlayerCommands::MoveRight),
            "MenuEscape" => event = Ok(PlayerCommands::MenuEscape),
            "ConnectToServer" => event = Ok(PlayerCommands::ConnectToServer),
            "DisconnectFromServer" => event = Ok(PlayerCommands::DisconnectFromServer),
            _ => {
                println!("Could not map {:?} to event", key);
                continue;
            }
        }

        match val.as_str().unwrap() {
            "W" => keycode = Ok(KeyCode::W),
            "A" => keycode = Ok(KeyCode::A),
            "S" => keycode = Ok(KeyCode::S),
            "D" => keycode = Ok(KeyCode::D),
            "Esc" => keycode = Ok(KeyCode::Escape),
            "Enter" => keycode = Ok(KeyCode::Return),
            _ => {
                println!("Could not map {:?} to Keycode", val);
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

pub struct HandleInputResult {
    pub generated_events: Vec<PlayerCommands>,
}

pub trait MappedInputHandler {
    fn handle_keyboard_input(
        &self,
        mapped_keyboard_events: MappedKeyboardInput,
    ) -> Vec<PlayerCommands>;
}

#[derive(Component)]
pub struct InputContext {
    is_active: bool,
    name: String,
    keyboard_input_map: KeyboardInputMap,
    pub handler: Box<dyn MappedInputHandler + Send + Sync>,
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

    /** name in constructor corresponds to table in .toml from which keybinds are read */
    pub fn new(handler: Box<dyn MappedInputHandler + Send + Sync>, name: String) -> InputContext {
        let filename = "conf.toml";
        let contents = match fs::read_to_string(filename) {
            // If successful return the files text as `contents`.
            // `c` is a local variable.
            Ok(c) => c,
            // Handle the `error` case.
            Err(err) => {
                // Write `msg` to `stderr`.
                println!("Could not read file `{}` {:?}", filename, err);
                panic!();
            }
        };

        // parse config into map
        let config = contents.parse::<Table>().unwrap();

        // Try to find table in config that maps to context handlers name
        let inputconfig: &Map<String, Value> = config.get(&name).unwrap().as_table().unwrap();

        let keyboard_input_map = create_input_map(inputconfig);

        return InputContext {
            is_active: false,
            keyboard_input_map,
            name,
            handler,
        };
    }

    pub fn handle_input(&self, keyboard_events: &mut Vec<&KeyboardInput>) -> HandleInputResult {
        // println!("Handling events for context: {:?}", self.name);
        let mut mapped_events: MappedKeyboardInput = vec![];

        // operate in place on the events passed to input context
        // removing the ones we consume
        keyboard_events.retain(|e| {
            let keycode = e.key_code.unwrap();
            if e.state == ButtonState::Released {
                return false;
            }
            if self.keyboard_input_map.contains_key(&keycode) == false {
                return true;
            }
            let event = self.keyboard_input_map.get(&keycode).unwrap().clone();
            mapped_events.push((*e, event));
            return false;
        });

        return HandleInputResult {
            generated_events: self.handler.handle_keyboard_input(mapped_events),
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

impl MappedInputHandler for GameInputContextHandler {
    fn handle_keyboard_input(&self, mapped_input: MappedKeyboardInput) -> Vec<PlayerCommands> {
        let mut events: Vec<PlayerCommands> = vec![];

        for (_, b) in mapped_input.into_iter() {
            events.push(b);
        }

        return events;
    }
}

// -- Secondary InputContextHandler
pub struct MenuInputContextHandler {}

impl MenuInputContextHandler {
    pub fn new() -> Box<MenuInputContextHandler> {
        return Box::new(MenuInputContextHandler {});
    }
}

impl MappedInputHandler for MenuInputContextHandler {
    fn handle_keyboard_input(&self, mapped_input: MappedKeyboardInput) -> Vec<PlayerCommands> {
        let mut events: Vec<PlayerCommands> = vec![];

        for (_, b) in mapped_input.into_iter() {
            events.push(b);
        }

        return events;
    }
}

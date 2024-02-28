use bevy::ecs::event::Event;
use serde::{Deserialize, Serialize};

#[derive(Event, Debug, Clone, Serialize, Deserialize)]
pub enum PlayerCommands {
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,

    MenuEscape,

    ConnectToServer,
    DisconnectFromServer,
}

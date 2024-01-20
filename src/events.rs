use bevy::ecs::event::Event;

#[derive(Event, Debug, Clone)]
pub enum GameEvents {
    MoveForward,
    MoveBack,
    MoveLeft,
    MoveRight,

    MenuEscape,
}

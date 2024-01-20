use bevy::ecs::event::Event;

#[derive(Event)]
pub enum GameEvents {
    MoveForward,
    MoveDown,
    MoveLeft,
    MoveRight,
}

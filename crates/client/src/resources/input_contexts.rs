use bevy::ecs::system::Resource;

use crate::components::input_context::InputContext;

#[derive(Resource)]
pub struct InputContexts {
    pub contexts: Vec<InputContext>,
}

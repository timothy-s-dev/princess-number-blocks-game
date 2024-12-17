use bevy::prelude::{KeyCode, Reflect};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::prelude::VirtualDPad;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    #[actionlike(DualAxis)]
    Move,
    Interact,
}

/// Specifies the keyboard/controller bindings for the actions available to the player
/// Separated out to keep the spawn function a bit cleaner.
pub fn get_input_map() -> InputMap<Action> {
    InputMap::default()
        .with(Action::Interact, KeyCode::Space)
        .with_dual_axis(
            Action::Move,
            VirtualDPad::new(KeyCode::KeyW, KeyCode::KeyS, KeyCode::KeyA, KeyCode::KeyD),
        )
}

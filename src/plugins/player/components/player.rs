use bevy::prelude::{ChildBuild, ChildBuilder, Component, KeyCode, Reflect};
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::prelude::VirtualDPad;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    #[actionlike(DualAxis)]
    Move,
    Interact,
}

#[derive(Component)]
pub struct Player;

pub fn spawn_player(parent: &mut ChildBuilder) {
    let input_map = InputMap::default()
        .with(Action::Interact, KeyCode::Space)
        .with_dual_axis(Action::Move, VirtualDPad::new(
            KeyCode::KeyW,
            KeyCode::KeyS,
            KeyCode::KeyA,
            KeyCode::KeyD,
        ));

    parent.spawn(InputManagerBundle::with_map(input_map)).insert(Player);
}

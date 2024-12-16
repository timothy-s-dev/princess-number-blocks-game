use bevy::prelude::{info, Query, With};
use leafwing_input_manager::prelude::ActionState;
use crate::plugins::player::components::player::{Action, Player};

pub fn player_movement_system(
    query: Query<&ActionState<Action>, With<Player>>
) {
    let action_state = query.single();
    if action_state.just_pressed(&Action::Interact) {
        info!("Player interacted");
    }
}
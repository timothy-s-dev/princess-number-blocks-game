use bevy::prelude::{in_state, App, IntoSystemConfigs, Update};
use crate::plugins::player::systems::player_animation::{
    player_animation_change_system,
    player_animation_tick_system,
};
use crate::plugins::player::systems::player_movement::player_movement_system;
use crate::scenes::GameState;

pub mod systems;
pub mod components;

pub fn player_plugin(app: &mut App) {
    app.add_systems(Update, (
        player_movement_system.run_if(in_state(GameState::Game)),
        player_animation_change_system.run_if(in_state(GameState::Game)),
        player_animation_tick_system.run_if(in_state(GameState::Game)),
    ));
}
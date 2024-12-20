use crate::animations::animation::Animation;
use crate::common_components::animation_timer::AnimationTimer;
use bevy::prelude::Component;

#[derive(Component, Debug, Default, Eq, PartialEq)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
}

#[derive(Component, Debug, Default, Eq, PartialEq)]
pub enum Facing {
    North,
    #[default]
    South,
    East,
    West,
}

/// This component is fairly complex, and should be spawned with the [spawn_player] function,
/// instead of being directly spawned or added to an existing entity.
#[derive(Component)]
#[require(PlayerState, Facing, AnimationTimer, Animation)]
pub struct Player;

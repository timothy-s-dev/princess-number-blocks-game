use crate::animations::animation::Animation;
use crate::common::components::animation_timer::AnimationTimer;
use crate::common::components::obstacle::Obstacle;
use crate::plugins::chest::components::chest_state::ChestState;
use bevy::prelude::Component;

#[derive(Component, Debug, Default)]
#[require(Obstacle, AnimationTimer, Animation, ChestState)]
pub struct Chest {
    pub contents: u32,
}

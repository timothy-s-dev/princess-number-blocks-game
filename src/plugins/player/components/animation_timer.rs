use bevy::prelude::{Component, Timer};

#[derive(Component, Default)]
pub struct AnimationTimer {
    pub timer: Timer,
}
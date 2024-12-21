use bevy::prelude::{Component, Timer};

#[derive(Component, Default)]
pub struct LifeTimer {
    pub timer: Timer,
}

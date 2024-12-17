use bevy::prelude::{Component, Timer};

/// This timer is set based on the framerate of an
/// [Animation](crate::animations::animation::Animation). It is ticked and monitored by the
/// [animation_plugin](crate::plugins::animation::animation_plugin), which also updates the Sprite.
#[derive(Component, Default)]
pub struct AnimationTimer {
    pub timer: Timer,
}

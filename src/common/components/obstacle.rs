use bevy::prelude::{Component, Sprite, Transform};

#[derive(Component, Default)]
#[require(Sprite, Transform)]
pub struct Obstacle;

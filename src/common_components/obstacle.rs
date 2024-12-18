use bevy::prelude::{Component, Sprite, Transform};

#[derive(Component)]
#[require(Sprite, Transform)]
pub struct Obstacle;

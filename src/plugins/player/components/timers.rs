use bevy::prelude::{Component, Timer};

#[derive(Component, Default)]
pub struct InteractTimer(pub Timer);

#[derive(Component, Default)]
pub struct DisplayTimer(pub Timer);

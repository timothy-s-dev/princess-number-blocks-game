use bevy::prelude::{Component, Entity};

#[derive(Component, Default)]
pub struct InteractionTarget(pub Option<Entity>);

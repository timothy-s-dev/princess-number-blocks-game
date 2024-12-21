use bevy::prelude::Component;

#[derive(Component, Eq, PartialEq, Debug, Clone, Copy, Hash, Default)]
pub enum ChestState {
    Opened,
    #[default]
    Closed,
    Opening,
    Closing,
}

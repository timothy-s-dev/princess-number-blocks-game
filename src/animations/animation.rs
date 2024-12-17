use bevy::prelude::Component;

#[derive(Component)]
pub struct Animation {
    pub start_index: usize,
    pub end_index: usize,
    pub framerate: u64,
    pub is_looping: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            start_index: 0,
            end_index: 0,
            framerate: 15,
            is_looping: true,
        }
    }
}

impl Animation {
    pub const fn from_single_frame(index: usize) -> Self {
        Self {
            start_index: index,
            end_index: index,
            framerate: 15,
            is_looping: false,
        }
    }

    pub const fn from_range(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
            framerate: 15,
            is_looping: true,
        }
    }
}

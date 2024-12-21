use bevy::prelude::Component;

/// Represents an animation on a sprite sheet.
///
/// Indexes start at 0 at the top left, and increase left-right, then top-bottom in the grid.
///
/// framerate is in frames-per-second and defaults to 15
///
/// is_looping determines the behavior when an animation tick would increment the index past
/// `end_index`. If false it will stop the animation at `end_index`, if true it loops back to
/// `start_index`
#[derive(Component)]
pub struct Animation {
    pub start_index: usize,
    pub end_index: usize,
    pub framerate: u64,
    pub is_looping: bool,
    pub reverse: bool,
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            start_index: 0,
            end_index: 0,
            framerate: 15,
            is_looping: true,
            reverse: false,
        }
    }
}

impl Animation {
    /// Returns an Animation struct for a single-frame "animation", non-looping
    pub const fn from_single_frame(index: usize) -> Self {
        Self {
            start_index: index,
            end_index: index,
            framerate: 15,
            is_looping: false,
            reverse: false,
        }
    }

    /// Returns a looping Animation struct for a range of frames using the default framerate (15)
    pub const fn from_range(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
            framerate: 15,
            is_looping: true,
            reverse: end_index < start_index,
        }
    }

    /// Returns a non-looping Animation struct for a range of frames using the default framerate (15)
    pub const fn from_range_once(start_index: usize, end_index: usize) -> Self {
        Self {
            start_index,
            end_index,
            framerate: 15,
            is_looping: false,
            reverse: end_index < start_index,
        }
    }
}

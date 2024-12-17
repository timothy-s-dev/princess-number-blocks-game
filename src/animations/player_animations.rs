use crate::animations::animation::Animation;

pub const IDLE_EAST: Animation = Animation::from_single_frame(32);
pub const IDLE_NORTH: Animation = Animation::from_single_frame(33);
pub const IDLE_WEST: Animation = Animation::from_single_frame(34);
pub const IDLE_SOUTH: Animation = Animation::from_single_frame(35);

pub const WALK_EAST: Animation = Animation::from_range(0, 7);
pub const WALK_NORTH: Animation = Animation::from_range(8, 15);
pub const WALK_WEST: Animation = Animation::from_range(16, 23);
pub const WALK_SOUTH: Animation = Animation::from_range(24, 31);

use crate::animations::animation::Animation;

pub const CLOSED: Animation = Animation::from_single_frame(0);
pub const OPENED: Animation = Animation::from_single_frame(9);

pub const OPENING: Animation = Animation::from_range_once(0, 9);
pub const CLOSING: Animation = Animation::from_range_once(9, 0);

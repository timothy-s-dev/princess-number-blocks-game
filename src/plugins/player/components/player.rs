use crate::animations::animation::Animation;
use crate::common_components::animation_timer::AnimationTimer;
use crate::plugins::player::components::interaction_target::InteractionTarget;
use crate::plugins::player::components::timers::{DisplayTimer, InteractTimer};
use bevy::prelude::{Component, Vec2, Vec3, Vec3Swizzles};

#[derive(Component, Debug, Default, Eq, PartialEq)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
    Interacting,
    Display,
}

#[derive(Component, Debug, Default, Eq, PartialEq)]
pub enum Facing {
    North,
    #[default]
    South,
    East,
    West,
}

#[derive(Component, Debug, Default)]
pub struct Displaying(pub u32);

impl Facing {
    // If both axes are the same magnitude (exact diagonal) default to horizontal facings
    pub fn from_vec2(vec2: Vec2) -> Self {
        if vec2.y.abs() > vec2.x.abs() {
            if vec2.y < 0. {
                Facing::South
            } else {
                Facing::North
            }
        } else if vec2.x < 0. {
            Facing::West
        } else {
            Facing::East
        }
    }
    pub fn from_vec3(vec3: Vec3) -> Self {
        Self::from_vec2(vec3.xy())
    }
}

/// This component is fairly complex, and should be spawned with the [spawn_player] function,
/// instead of being directly spawned or added to an existing entity.
#[derive(Component)]
#[require(
    PlayerState,
    Facing,
    AnimationTimer,
    Animation,
    InteractTimer,
    DisplayTimer,
    InteractionTarget,
    Displaying
)]
pub struct Player;

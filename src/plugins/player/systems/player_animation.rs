use crate::animations::animation::Animation;
use crate::animations::player_animations;
use crate::plugins::player::components::animation_timer::AnimationTimer;
use crate::plugins::player::components::player::{Facing, Player, PlayerState};
use bevy::ecs::query::QueryData;
use bevy::prelude::{Changed, Or, Query, Sprite, Timer, TimerMode, With};
use std::time::Duration;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerAnimationChangeQuery {
    state: &'static PlayerState,
    facing: &'static Facing,
    animation_timer: &'static mut AnimationTimer,
    sprite: &'static mut Sprite,
    animation: &'static mut Animation,
}

type PlayerAnimationChangeFilter = (With<Player>, Or<(Changed<PlayerState>, Changed<Facing>)>);

/// This system watches for changes in the [PlayerState] and [Facing] of the player entity.
/// Any time such a change is detected, it replaces the [Animation] component with a new one,
/// based on the state and facing.
pub fn player_animation_change_system(
    mut query: Query<PlayerAnimationChangeQuery, PlayerAnimationChangeFilter>,
) {
    if let Ok(mut query_data) = query.get_single_mut() {
        let new_animation = match (query_data.state, query_data.facing) {
            (PlayerState::Idle, Facing::North) => player_animations::IDLE_NORTH,
            (PlayerState::Idle, Facing::South) => player_animations::IDLE_SOUTH,
            (PlayerState::Idle, Facing::East) => player_animations::IDLE_EAST,
            (PlayerState::Idle, Facing::West) => player_animations::IDLE_WEST,
            (PlayerState::Walking, Facing::North) => player_animations::WALK_NORTH,
            (PlayerState::Walking, Facing::South) => player_animations::WALK_SOUTH,
            (PlayerState::Walking, Facing::East) => player_animations::WALK_EAST,
            (PlayerState::Walking, Facing::West) => player_animations::WALK_WEST,
        };
        set_animation(new_animation, &mut query_data);
    }
}

fn set_animation(new_animation: Animation, query_data: &mut PlayerAnimationChangeQueryItem) {
    *query_data.animation = new_animation;
    query_data.animation_timer.timer = Timer::new(
        Duration::from_millis(1000 / query_data.animation.framerate),
        TimerMode::Repeating,
    );
    if let Some(atlas) = &mut query_data.sprite.texture_atlas {
        atlas.index = query_data.animation.start_index;
    }
}

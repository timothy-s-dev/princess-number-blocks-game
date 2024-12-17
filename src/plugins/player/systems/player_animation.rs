use std::time::Duration;
use bevy::ecs::query::QueryData;
use bevy::prelude::{Changed, Component, Or, Query, Res, Sprite, Time, Timer, TimerMode, With};
use crate::plugins::player::components::animation_timer::AnimationTimer;
use crate::plugins::player::components::player::{Facing, Player, PlayerState};

#[derive(Component)]
pub struct PlayerAnimation {
    start_index: usize,
    end_index: usize,
    framerate: u64,
    is_looping: bool,
}

const IDLE_ANIMATION: PlayerAnimation = PlayerAnimation {
    start_index: 32,
    end_index: 32,
    framerate: 15,
    is_looping: false,
};

impl Default for PlayerAnimation {
    fn default() -> Self { IDLE_ANIMATION }
}

const WALK_ANIMATION: PlayerAnimation = PlayerAnimation {
    start_index: 0,
    end_index: 7,
    framerate: 15,
    is_looping: true,
};

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerAnimationChangeQuery {
    state: &'static PlayerState,
    facing: &'static Facing,
    animation_timer: &'static mut AnimationTimer,
    sprite: &'static mut Sprite,
    animation: &'static mut PlayerAnimation,
}

fn set_animation(new_animation: PlayerAnimation, query_data: &mut PlayerAnimationChangeQueryItem) {
    *query_data.animation = new_animation;
    query_data.animation_timer.timer = Timer::new(
        Duration::from_millis(1000 / query_data.animation.framerate),
        TimerMode::Repeating
    );
    if let Some(atlas) = &mut query_data.sprite.texture_atlas {
        atlas.index = query_data.animation.start_index;
    }
}

pub fn player_animation_change_system(
    mut query: Query<PlayerAnimationChangeQuery, (With<Player>, Or<(Changed<PlayerState>, Changed<Facing>)>)>
) {
    if let Ok(mut query_data) = query.get_single_mut() {
        if *query_data.state == PlayerState::Idle {
            set_animation(IDLE_ANIMATION, &mut query_data);
        } else {
            set_animation(WALK_ANIMATION, &mut query_data);
        }
    }
}

pub fn player_animation_tick_system(
    mut query: Query<(&PlayerAnimation, &mut AnimationTimer, &mut Sprite), With<Player>>,
    time: Res<Time>
) {
    let (
        animation,
        mut animation_timer,
        mut sprite,
    ) = query.single_mut();

    animation_timer.timer.tick(time.delta());
    if animation_timer.timer.finished() {
        if let Some(atlas) = &mut sprite.texture_atlas {
            if atlas.index == animation.end_index && animation.is_looping {
                atlas.index = animation.start_index;
            } else if atlas.index < animation.end_index {
                atlas.index += 1;
            }
        }
        animation_timer.timer.reset();
    }
}
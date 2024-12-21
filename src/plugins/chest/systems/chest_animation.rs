use crate::animations::animation::Animation;
use crate::animations::chest_animations;
use crate::common_components::animation_timer::AnimationTimer;
use crate::plugins::animation::AnimationFinishedEvent;
use crate::plugins::chest::components::chest::Chest;
use crate::plugins::chest::components::chest_state::ChestState;
use crate::plugins::chest::{ChestOpenedEvent, CloseChestsEvent};
use bevy::ecs::query::QueryData;
use bevy::prelude::{DetectChanges, EventReader, EventWriter, Query, Sprite, Timer, TimerMode};
use std::time::Duration;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct ChestAnimationChangeQuery {
    chest: &'static Chest,
    state: &'static mut ChestState,
    animation_timer: &'static mut AnimationTimer,
    sprite: &'static mut Sprite,
    animation: &'static mut Animation,
}

/// This system watches for changes in the [ChestState] of each chest entity.
/// Any time such a change is detected, it replaces the [Animation] component with a new one,
/// based on the state. It also watches for finished animations and transitions accordingly.
pub fn chest_animation_change_system(
    mut chests: Query<ChestAnimationChangeQuery>,
    mut ev_animation_finished: EventReader<AnimationFinishedEvent>,
    mut ev_chest_opened: EventWriter<ChestOpenedEvent>,
    mut ev_close_chests: EventReader<CloseChestsEvent>,
) {
    let should_close_chests = !ev_close_chests.is_empty();
    if should_close_chests {
        ev_close_chests.clear();
    }
    for mut query_data in chests.iter_mut() {
        if query_data.state.is_changed() {
            let new_animation = match *query_data.state {
                ChestState::Closed => chest_animations::CLOSED,
                ChestState::Opening => chest_animations::OPENING,
                ChestState::Opened => chest_animations::OPENED,
                ChestState::Closing => chest_animations::CLOSING,
            };
            set_animation(new_animation, &mut query_data);
        }
        if should_close_chests && *query_data.state == ChestState::Opened {
            *query_data.state = ChestState::Closing;
            set_animation(chest_animations::CLOSING, &mut query_data);
        }
    }

    for event in ev_animation_finished.read() {
        let Ok(mut chest) = chests.get_mut(event.0) else {
            continue; // Don't care about animation finished events that aren't for chests
        };
        if *chest.state == ChestState::Opening {
            *chest.state = ChestState::Opened;
            ev_chest_opened.send(ChestOpenedEvent(chest.chest.contents));
        } else if *chest.state == ChestState::Closing {
            *chest.state = ChestState::Closed;
        }
    }
}

fn set_animation(new_animation: Animation, query_data: &mut ChestAnimationChangeQueryItem) {
    *query_data.animation = new_animation;
    query_data.animation_timer.timer = Timer::new(
        Duration::from_millis(1000 / query_data.animation.framerate),
        TimerMode::Repeating,
    );
    if let Some(atlas) = &mut query_data.sprite.texture_atlas {
        atlas.index = query_data.animation.start_index;
    }
}

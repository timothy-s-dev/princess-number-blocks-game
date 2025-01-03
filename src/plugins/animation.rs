use crate::animations::animation::Animation;
use crate::common::components::animation_timer::AnimationTimer;
use bevy::app::{App, Update};
use bevy::ecs::query::QueryData;
use bevy::prelude::{Entity, Event, EventWriter, Query, Res, Sprite, Time};

/// Automatically updates sprites/texture_atlases on a timer
///
/// This plugin adds a single system which queries for all entities with an [Animation], [Sprite],
/// and [AnimationTimer], and updates the timer and monitors it for completion. When the timer
/// completes, the sprite is updated to the next frame based on the animation, and the timer is
/// restarted.
pub fn animation_plugin(app: &mut App) {
    app.add_systems(Update, animation_tick_system);
    app.add_event::<AnimationFinishedEvent>();
}

#[derive(QueryData)]
#[query_data(mutable)]
struct AnimationTickQuery {
    entity: Entity,
    animation: &'static Animation,
    animation_timer: &'static mut AnimationTimer,
    sprite: &'static mut Sprite,
}

#[derive(Event, Debug)]
pub struct AnimationFinishedEvent(pub Entity);

fn animation_tick_system(
    mut query: Query<AnimationTickQuery>,
    time: Res<Time>,
    mut ev_animation_finished: EventWriter<AnimationFinishedEvent>,
) {
    for mut query_data in query.iter_mut() {
        query_data.animation_timer.timer.tick(time.delta());
        if query_data.animation_timer.timer.finished() {
            if let Some(atlas) = &mut query_data.sprite.texture_atlas {
                if atlas.index < query_data.animation.end_index && !query_data.animation.reverse {
                    atlas.index += 1;
                } else if atlas.index > 0 && query_data.animation.reverse {
                    atlas.index -= 1;
                } else if query_data.animation.is_looping {
                    atlas.index = query_data.animation.start_index;
                } else {
                    ev_animation_finished.send(AnimationFinishedEvent(query_data.entity));
                    query_data.animation_timer.timer.pause();
                }
            }
            query_data.animation_timer.timer.reset();
        }
    }
}

use bevy::app::{App, Update};
use bevy::ecs::query::QueryData;
use bevy::prelude::{Query, Res, Sprite, Time};
use crate::animations::animation::Animation;
use crate::plugins::player::components::animation_timer::AnimationTimer;

pub fn animation_plugin(app: &mut App) {
    app.add_systems(Update, animation_tick_system);
}

#[derive(QueryData)]
#[query_data(mutable)]
struct AnimationTickQuery {
    animation: &'static Animation,
    animation_timer: &'static mut AnimationTimer,
    sprite: &'static mut Sprite,
}

fn animation_tick_system(
    mut query: Query<AnimationTickQuery>,
    time: Res<Time>
) {
    for mut query_data in query.iter_mut() {
        query_data.animation_timer.timer.tick(time.delta());
        if query_data.animation_timer.timer.finished() {
            if let Some(atlas) = &mut query_data.sprite.texture_atlas {
                if atlas.index == query_data.animation.end_index && query_data.animation.is_looping {
                    atlas.index = query_data.animation.start_index;
                } else if atlas.index < query_data.animation.end_index {
                    atlas.index += 1;
                }
            }
            query_data.animation_timer.timer.reset();
        }
    }
}
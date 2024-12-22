use crate::common::components::life_timer::LifeTimer;
use bevy::prelude::{Commands, DespawnRecursiveExt, Entity, Query, Res, Time};

pub fn lifespan_tick_system(
    mut query: Query<(Entity, &mut LifeTimer)>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for mut query_data in query.iter_mut() {
        query_data.1.timer.tick(time.delta());
        if query_data.1.timer.finished() {
            commands.entity(query_data.0).despawn_recursive();
        }
    }
}

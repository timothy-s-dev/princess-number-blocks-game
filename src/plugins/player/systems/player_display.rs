use crate::plugins::chest::{ChestOpenedEvent, CloseChestsEvent};
use crate::plugins::player::components::player::{Displaying, Player, PlayerState};
use crate::plugins::player::components::timers::DisplayTimer;
use bevy::ecs::query::QueryData;
use bevy::prelude::{EventReader, EventWriter, Query, Res, Time, Timer, TimerMode, With};
use std::time::Duration;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerQuery {
    state: &'static mut PlayerState,
    display_timer: &'static mut DisplayTimer,
    displaying: &'static mut Displaying,
}

/// This system monitors the ChestOpenedEvent stream and the state on the player to shift the player
/// into and out of the Display state, and updates the value to be displayed accordingly.
pub fn player_display_system(
    mut query: Query<PlayerQuery, With<Player>>,
    mut ev_chest_opened: EventReader<ChestOpenedEvent>,
    mut ev_close_chests: EventWriter<CloseChestsEvent>,
    time: Res<Time>,
) {
    let mut player_data = query.single_mut();

    if *player_data.state == PlayerState::Display {
        player_data.display_timer.0.tick(time.delta());
        if player_data.display_timer.0.finished() {
            *player_data.state = PlayerState::Idle;
            player_data.displaying.0 = 0;
            ev_close_chests.send(CloseChestsEvent);
        }
    }

    for event in ev_chest_opened.read() {
        player_data.display_timer.0 = Timer::new(Duration::from_secs(3), TimerMode::Once);
        *player_data.state = PlayerState::Display;
        player_data.displaying.0 = event.0;
    }
}

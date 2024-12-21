use crate::animations::numberblock_animations::{
    get_numberblock_texture_atlas_layout, NUMBERBLOCK_SPRITE_SHEET,
};
use crate::plugins::chest::{ChestOpenedEvent, CloseChestsEvent};
use crate::plugins::numberblock::spawn_numberblock;
use crate::plugins::player::components::player::{Displaying, Player, PlayerState};
use crate::plugins::player::components::timers::DisplayTimer;
use bevy::asset::{AssetServer, Assets};
use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::ecs::query::QueryData;
use bevy::prelude::{
    EventReader, EventWriter, Query, Res, ResMut, TextureAtlasLayout, Time, Timer, TimerMode,
    Transform, With,
};
use std::time::Duration;

pub const DISPLAY_TIME: u64 = 1500;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct PlayerQuery {
    state: &'static mut PlayerState,
    display_timer: &'static mut DisplayTimer,
    displaying: &'static mut Displaying,
    transform: &'static mut Transform,
}

/// This system monitors the ChestOpenedEvent stream and the state on the player to shift the player
/// into and out of the Display state, and updates the value to be displayed accordingly.
pub fn player_display_system(
    mut query: Query<PlayerQuery, With<Player>>,
    mut ev_chest_opened: EventReader<ChestOpenedEvent>,
    mut ev_close_chests: EventWriter<CloseChestsEvent>,
    time: Res<Time>,
    mut commands: bevy::prelude::Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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
        player_data.display_timer.0 =
            Timer::new(Duration::from_millis(DISPLAY_TIME), TimerMode::Once);
        *player_data.state = PlayerState::Display;
        player_data.displaying.0 = event.0;
        let numberblock_pos =
            player_data.transform.translation - Transform::from_xyz(0., -200.0, 0.).translation;
        let texture = asset_server.load(NUMBERBLOCK_SPRITE_SHEET);
        let layout = get_numberblock_texture_atlas_layout();
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        spawn_numberblock(
            &mut commands,
            texture,
            texture_atlas_layout,
            numberblock_pos.x,
            numberblock_pos.y,
            event.0,
            Some(DISPLAY_TIME),
        );

        // TODO: This should move to whatever system handles challenges/questions/problems
        commands
            .spawn(AudioPlayer::new(asset_server.load("sfx/correct.wav")))
            .insert(PlaybackSettings::DESPAWN.with_volume(Volume::new(0.25)));
    }
}

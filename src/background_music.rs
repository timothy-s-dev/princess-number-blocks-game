use bevy::prelude::{AssetServer, Handle, Res, ResMut, Resource};
use bevy_kira_audio::prelude::*;
use rand::Rng;

const ALL_ASSETS: &[&str] = &[
    "music/Cheery Monday.mp3",
    "music/Monkeys Spinning Monkeys.mp3",
    "music/Moonlight Beach.mp3",
    "music/On My Way.mp3",
    "music/Rainbows.mp3",
    "music/Teddy Bear Waltz.mp3",
];

fn get_random_song() -> &'static str {
    ALL_ASSETS[rand::thread_rng().gen_range(0..ALL_ASSETS.len())]
}

#[derive(Resource)]
pub struct BackgroundAudio(pub Handle<AudioInstance>);

pub fn start_background_audio(
    asset_server: Res<AssetServer>,
    mut background_audio: ResMut<BackgroundAudio>,
    audio: Res<Audio>,
) {
    let song = get_random_song();
    background_audio.0 = audio
        .play(asset_server.load(song))
        .with_volume(0.25)
        .handle();
}

pub fn restart_background_audio(
    asset_server: Res<AssetServer>,
    mut background_audio: ResMut<BackgroundAudio>,
    audio: Res<Audio>,
) {
    let status = audio.state(&background_audio.0);
    if status != PlaybackState::Stopped {
        return;
    }
    let song = get_random_song();
    background_audio.0 = audio
        .play(asset_server.load(song))
        .with_volume(0.25)
        .handle();
}

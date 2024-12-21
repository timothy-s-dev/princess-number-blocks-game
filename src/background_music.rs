use bevy::audio::Volume;
use bevy::prelude::{
    debug, AssetServer, AudioPlayer, Commands, Component, Entity, PlaybackSettings, Query, Res,
    With, Without,
};
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

#[derive(Component)]
pub struct BackgroundAudio;

pub fn start_background_audio(asset_server: Res<AssetServer>, mut commands: Commands) {
    let song = get_random_song();
    debug!("Now playing: {}", song);
    commands
        .spawn(BackgroundAudio)
        .insert(AudioPlayer::new(asset_server.load(song)))
        .insert(PlaybackSettings::REMOVE.with_volume(Volume::new(0.25)));
}

pub fn restart_background_audio(
    audio_player: Query<Entity, (With<BackgroundAudio>, Without<AudioPlayer>)>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    let Ok(audio_player) = audio_player.get_single() else {
        return;
    };
    let song = get_random_song();
    debug!("Now playing: {}", song);
    commands
        .entity(audio_player)
        .insert(AudioPlayer::new(asset_server.load(song)))
        .insert(PlaybackSettings::REMOVE.with_volume(Volume::new(0.25)));
}

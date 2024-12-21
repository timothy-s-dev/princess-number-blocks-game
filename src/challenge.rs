use crate::plugins::chest::ChestOpenedEvent;
use bevy::asset::AssetServer;
use bevy::audio::{AudioPlayer, PlaybackSettings, Volume};
use bevy::prelude::{debug, Commands, EventReader, Res, ResMut, Resource};
use rand::Rng;

#[derive(Resource)]
pub struct Challenge {
    pub addend1: u32,
    pub addend2: u32,
    pub sum: u32,
}

impl Challenge {
    pub fn randomized() -> Self {
        let sum = rand::thread_rng().gen_range(2..10);
        let addend1 = rand::thread_rng().gen_range(1..sum);
        let addend2 = sum - addend1;
        // TODO: Remove once HUD is done
        debug!("Challenge: {} + {} = {}", addend1, addend2, sum);
        Self {
            addend1,
            addend2,
            sum,
        }
    }
}

pub fn initialize_challenge_system(mut commands: Commands) {
    commands.insert_resource(Challenge::randomized());
}

pub fn monitor_challenge_system(
    mut ev_chest_opened: EventReader<ChestOpenedEvent>,
    mut challenge: ResMut<Challenge>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    for event in ev_chest_opened.read() {
        if challenge.sum == event.0 {
            commands
                .spawn(AudioPlayer::new(asset_server.load("sfx/correct.wav")))
                .insert(PlaybackSettings::DESPAWN.with_volume(Volume::new(0.25)));
            *challenge = Challenge::randomized();
        } else {
            commands
                .spawn(AudioPlayer::new(asset_server.load("sfx/wrong.wav")))
                .insert(PlaybackSettings::DESPAWN.with_volume(Volume::new(0.25)));
        }
    }
}

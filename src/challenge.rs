use crate::animations::numberblock_animations::{
    get_numberblock_texture_atlas_layout, NUMBERBLOCK_SPRITE_SHEET,
};
use crate::plugins::chest::ChestOpenedEvent;
use crate::plugins::numberblock::spawn_numberblock;
use bevy::asset::{AssetServer, Assets};
use bevy::prelude::{
    Commands, Component, DespawnRecursiveExt, DetectChanges, Entity, EventReader, Query, Res,
    ResMut, Resource, TextureAtlasLayout, With,
};
use bevy_kira_audio::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct ChallengeHud;

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
    challenge: Res<Challenge>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    for event in ev_chest_opened.read() {
        if challenge.sum == event.0 {
            audio
                .play(asset_server.load("sfx/correct.wav"))
                .with_volume(0.25);
        } else {
            audio
                .play(asset_server.load("sfx/wrong.wav"))
                .with_volume(0.5);
        }
    }
}

pub fn challenge_hud_system(
    challenge: Res<Challenge>,
    mut commands: Commands,
    hud_entities: Query<Entity, With<ChallengeHud>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if challenge.is_changed() {
        for entity in hud_entities.iter() {
            commands.entity(entity).despawn_recursive();
        }

        let texture = asset_server.load(NUMBERBLOCK_SPRITE_SHEET);
        let layout = get_numberblock_texture_atlas_layout();
        let texture_atlas_layout = texture_atlas_layouts.add(layout);

        let addend1_block = spawn_numberblock(
            &mut commands,
            texture.clone(),
            texture_atlas_layout.clone(),
            -600.,
            -200.,
            challenge.addend1,
            None,
        );
        commands.entity(addend1_block).insert(ChallengeHud);

        let addend2_block = spawn_numberblock(
            &mut commands,
            texture.clone(),
            texture_atlas_layout.clone(),
            -550.,
            -200.,
            challenge.addend2,
            None,
        );
        commands.entity(addend2_block).insert(ChallengeHud);
    }
}

use crate::animations::animation::Animation;
use crate::animations::numberblock_animations;
use crate::common::components::animation_timer::AnimationTimer;
use crate::common::components::life_timer::LifeTimer;
use bevy::prelude::{
    default, Commands, Component, Entity, Handle, Image, Sprite, TextureAtlas, TextureAtlasLayout,
    Timer, TimerMode, Transform,
};
use std::time::Duration;

#[derive(Component)]
#[require(AnimationTimer, Animation)]
pub struct Numberblock;

fn get_numberblock_animation(value: u32) -> Animation {
    match value {
        1 => numberblock_animations::ONE,
        2 => numberblock_animations::TWO,
        3 => numberblock_animations::THREE,
        4 => numberblock_animations::FOUR,
        5 => numberblock_animations::FIVE,
        6 => numberblock_animations::SIX,
        7 => numberblock_animations::SEVEN,
        8 => numberblock_animations::EIGHT,
        9 => numberblock_animations::NINE,
        10 => numberblock_animations::TEN,
        _ => panic!("Invalid value for numberblock"),
    }
}

pub fn spawn_numberblock(
    commands: &mut Commands,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
    x: f32,
    y: f32,
    value: u32,
    lifetime: Option<u64>,
) -> Entity {
    let mut entity = commands.spawn(Numberblock);
    entity
        .insert(Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: (value - 1) as usize,
            }),
            ..default()
        })
        .insert(Transform::from_xyz(x, y, 100.))
        .insert(get_numberblock_animation(value));
    if let Some(lifetime) = lifetime {
        entity.insert(LifeTimer {
            timer: Timer::new(Duration::from_millis(lifetime), TimerMode::Repeating),
        });
    }
    entity.id()
}

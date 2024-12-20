use crate::animations::animation::Animation;
use crate::common_components::animation_timer::AnimationTimer;
use crate::common_components::obstacle::Obstacle;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{
    default, ChildBuild, ChildBuilder, Component, Handle, Image, Sprite, TextureAtlas,
    TextureAtlasLayout, Transform,
};

pub const CHEST_SPRITE_SHEET: &str = "chest.png";
pub fn get_chest_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(UVec2::new(48, 32), 5, 6, None, None)
}

#[derive(Component)]
pub enum ChestState {
    Opened,
    Closed,
    Opening,
    Closing,
}

#[derive(Component)]
#[require(Obstacle, AnimationTimer, Animation)]
pub struct Chest {
    contents: u32,
}

pub fn spawn_chest(
    parent: &mut ChildBuilder,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
    x: f32,
    y: f32,
    contents: u32,
) {
    parent
        .spawn(Chest { contents })
        .insert(Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::new(48., 32.)),
            ..default()
        })
        .insert(Transform::from_xyz(x, y, 0.));
}

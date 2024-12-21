use crate::animations::animation::Animation;
use bevy::math::UVec2;
use bevy::prelude::TextureAtlasLayout;

pub const NUMBERBLOCK_SPRITE_SHEET: &str = "numberblocks.png";

pub fn get_numberblock_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(UVec2::new(32, 320), 10, 1, None, None)
}

pub const ONE: Animation = Animation::from_single_frame(0);
pub const TWO: Animation = Animation::from_single_frame(1);
pub const THREE: Animation = Animation::from_single_frame(2);
pub const FOUR: Animation = Animation::from_single_frame(3);
pub const FIVE: Animation = Animation::from_single_frame(4);
pub const SIX: Animation = Animation::from_single_frame(5);
pub const SEVEN: Animation = Animation::from_single_frame(6);
pub const EIGHT: Animation = Animation::from_single_frame(7);
pub const NINE: Animation = Animation::from_single_frame(8);
pub const TEN: Animation = Animation::from_single_frame(9);

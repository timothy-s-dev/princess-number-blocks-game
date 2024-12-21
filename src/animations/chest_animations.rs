use crate::animations::animation::Animation;
use bevy::math::UVec2;
use bevy::prelude::TextureAtlasLayout;

pub const CHEST_SPRITE_SHEET: &str = "chest.png";

pub fn get_chest_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(UVec2::new(48, 32), 5, 6, None, None)
}

pub const CLOSED: Animation = Animation::from_single_frame(0);
pub const OPENED: Animation = Animation::from_single_frame(9);

pub const OPENING: Animation = Animation::from_range_once(0, 9);
pub const CLOSING: Animation = Animation::from_range_once(9, 0);

use crate::animations::animation::Animation;
use bevy::math::UVec2;
use bevy::prelude::TextureAtlasLayout;

// This file contains the definitions for the various animations used by the player entity.

pub const PLAYER_SPRITE_SHEET: &str = "princess.png";

pub fn get_player_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(UVec2::splat(24), 8, 9, None, None)
}

pub const IDLE_EAST: Animation = Animation::from_single_frame(32);
pub const IDLE_NORTH: Animation = Animation::from_single_frame(33);
pub const IDLE_WEST: Animation = Animation::from_single_frame(34);
pub const IDLE_SOUTH: Animation = Animation::from_single_frame(35);

pub const WALK_EAST: Animation = Animation::from_range(0, 7);
pub const WALK_NORTH: Animation = Animation::from_range(8, 15);
pub const WALK_WEST: Animation = Animation::from_range(16, 23);
pub const WALK_SOUTH: Animation = Animation::from_range(24, 31);

pub const INTERACT_EAST: Animation = Animation::from_single_frame(44);
pub const INTERACT_NORTH: Animation = Animation::from_single_frame(52);
pub const INTERACT_WEST: Animation = Animation::from_single_frame(60);
pub const INTERACT_SOUTH: Animation = Animation::from_single_frame(68);

pub const DISPLAY_EAST: Animation = Animation::from_range_once(40, 42);
pub const DISPLAY_NORTH: Animation = Animation::from_range_once(48, 50);
pub const DISPLAY_WEST: Animation = Animation::from_range_once(56, 58);
pub const DISPLAY_SOUTH: Animation = Animation::from_range_once(64, 66);

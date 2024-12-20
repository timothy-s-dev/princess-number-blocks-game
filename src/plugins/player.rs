use crate::input_map::get_input_map;
use crate::plugins::player::components::player::Player;
use crate::plugins::player::systems::player_animation::player_animation_change_system;
use crate::plugins::player::systems::player_movement::player_movement_system;
use crate::scenes::GameState;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{
    default, in_state, App, ChildBuild, ChildBuilder, Handle, Image, IntoSystemConfigs, Sprite,
    TextureAtlas, TextureAtlasLayout, Update,
};
use leafwing_input_manager::InputManagerBundle;

pub mod components;
pub mod systems;

pub const PLAYER_SPRITE_SHEET: &str = "princess.png";
pub fn get_player_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(UVec2::splat(24), 8, 9, None, None)
}

pub fn player_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            player_movement_system.run_if(in_state(GameState::Game)),
            player_animation_change_system.run_if(in_state(GameState::Game)),
        ),
    );
}

/// The player entity is relatively complex, and so instead of relying entirely on `require`ing
/// all the related components, this helper function is used to spawn it. It requires a parent,
/// so that when the scene changes it can be cleaned up properly.
///
/// Some components ([PlayerState], [Facing], etc) are simple enough that they are provided a
/// Default implementation and simply `require`d by the Player component.
pub fn spawn_player(
    parent: &mut ChildBuilder,
    texture: Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
) {
    parent
        .spawn(InputManagerBundle::with_map(get_input_map()))
        .insert(Player)
        .insert(Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            custom_size: Some(Vec2::new(48., 48.)),
            ..default()
        });
}

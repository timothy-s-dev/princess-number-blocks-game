use crate::plugins::chest::components::chest::Chest;
use crate::plugins::chest::systems::chest_animation::chest_animation_change_system;
use crate::scenes::GameState;
use bevy::app::App;
use bevy::math::{UVec2, Vec2};
use bevy::prelude::{
    default, in_state, ChildBuild, ChildBuilder, Event, Handle, Image, IntoSystemConfigs, Sprite,
    TextureAtlas, TextureAtlasLayout, Transform, Update,
};

pub mod components;
pub mod systems;

pub fn chest_plugin(app: &mut App) {
    app.add_systems(
        Update,
        (chest_animation_change_system.run_if(in_state(GameState::Game)),),
    );
    app.add_event::<ChestOpenedEvent>();
    app.add_event::<CloseChestsEvent>();
}

#[derive(Event)]
pub struct ChestOpenedEvent(pub u32);
#[derive(Event)]
pub struct CloseChestsEvent;

pub const CHEST_SPRITE_SHEET: &str = "chest.png";
pub fn get_chest_texture_atlas_layout() -> TextureAtlasLayout {
    TextureAtlasLayout::from_grid(UVec2::new(48, 32), 5, 6, None, None)
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

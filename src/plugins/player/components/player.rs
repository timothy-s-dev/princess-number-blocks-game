use crate::animations::animation::Animation;
use crate::input_map::get_input_map;
use crate::plugins::player::components::animation_timer::AnimationTimer;
use bevy::prelude::{
    default, AssetServer, Assets, ChildBuild, ChildBuilder, Component, Res, ResMut, Sprite,
    TextureAtlas, TextureAtlasLayout, UVec2,
};
use leafwing_input_manager::InputManagerBundle;

#[derive(Component, Debug, Default, Eq, PartialEq)]
pub enum PlayerState {
    #[default]
    Idle,
    Walking,
}

#[derive(Component, Debug, Default, Eq, PartialEq)]
pub enum Facing {
    North,
    #[default]
    South,
    East,
    West,
}

/// This component is fairly complex, and should be spawned with the [spawn_player] function,
/// instead of being directly spawned or added to an existing entity.
#[derive(Component)]
#[require(PlayerState, Facing, AnimationTimer, Animation)]
pub struct Player;

/// The player entity is relatively complex, and so instead of relying entirely on `require`ing
/// all the related components, this helper function is used to spawn it. It requires a parent,
/// so that when the scene changes it can be cleaned up properly.
///
/// Some components ([PlayerState], [Facing], etc) are simple enough that they are provided a
/// Default implementation and simply `require`d by the Player component.
pub fn spawn_player(
    parent: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("princess.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 8, 9, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    parent
        .spawn(InputManagerBundle::with_map(get_input_map()))
        .insert(Player)
        .insert(Sprite {
            image: texture.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: texture_atlas_layout.clone(),
                index: 0,
            }),
            ..default()
        });
}

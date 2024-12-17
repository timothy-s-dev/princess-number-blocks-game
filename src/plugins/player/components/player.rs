use bevy::prelude::{default, AssetServer, Assets, ChildBuild, ChildBuilder, Component, KeyCode, Reflect, Res, ResMut, Sprite, TextureAtlas, TextureAtlasLayout, UVec2};
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::prelude::VirtualDPad;
use crate::plugins::player::components::animation_timer::AnimationTimer;
use crate::animations::animation::Animation;


#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum Action {
    #[actionlike(DualAxis)]
    Move,
    Interact,
}

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
    West
}

#[derive(Component)]
#[require(PlayerState, Facing, AnimationTimer, Animation)]
pub struct Player;

pub fn spawn_player(
    parent: &mut ChildBuilder,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    let input_map = InputMap::default()
        .with(Action::Interact, KeyCode::Space)
        .with_dual_axis(Action::Move, VirtualDPad::new(
            KeyCode::KeyW,
            KeyCode::KeyS,
            KeyCode::KeyA,
            KeyCode::KeyD,
        ));

    let texture = asset_server.load("princess.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(24), 8, 9, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    parent.spawn(InputManagerBundle::with_map(input_map))
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

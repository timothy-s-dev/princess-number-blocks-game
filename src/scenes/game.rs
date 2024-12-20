use super::{despawn_screen, GameState};
use crate::common_components::obstacle::Obstacle;
use crate::plugins::chest::{get_chest_texture_atlas_layout, spawn_chest, CHEST_SPRITE_SHEET};
use crate::plugins::player::{get_player_texture_atlas_layout, spawn_player, PLAYER_SPRITE_SHEET};
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup)
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_texture = asset_server.load(PLAYER_SPRITE_SHEET);
    let player_layout = get_player_texture_atlas_layout();
    let player_texture_atlas_layout = texture_atlas_layouts.add(player_layout);

    let chest_texture = asset_server.load(CHEST_SPRITE_SHEET);
    let chest_layout = get_chest_texture_atlas_layout();
    let chest_texture_atlas_layout = texture_atlas_layouts.add(chest_layout);

    commands
        .spawn(OnGameScreen)
        .insert(Transform::default())
        .insert(InheritedVisibility::default())
        .with_children(|parent| {
            for x in -20..20 {
                spawn_obstacle(parent, &asset_server, 32. * x as f32 + 16., -360. + 16.);
                spawn_obstacle(parent, &asset_server, 32. * x as f32 + 16., 360. - 16.);
            }
            for y in -11..11 {
                spawn_obstacle(parent, &asset_server, -640. + 16., 32. * y as f32 + 16.);
                spawn_obstacle(parent, &asset_server, 640. - 16., 32. * y as f32 + 16.);
            }
            spawn_chest(
                parent,
                chest_texture.clone(),
                chest_texture_atlas_layout.clone(),
                100.,
                100.,
                1,
            );
            spawn_chest(
                parent,
                chest_texture.clone(),
                chest_texture_atlas_layout.clone(),
                200.,
                200.,
                1,
            );
            spawn_player(
                parent,
                player_texture.clone(),
                player_texture_atlas_layout.clone(),
            );
        });
}

fn game() {
    // TODO?
}

fn spawn_obstacle(parent: &mut ChildBuilder, asset_server: &Res<AssetServer>, x: f32, y: f32) {
    let texture = asset_server.load("rock.png");
    parent
        .spawn(Obstacle)
        .insert(Sprite {
            image: texture.clone(),
            custom_size: Some(Vec2::new(32., 32.)),
            ..default()
        })
        .insert(Transform::from_xyz(x, y, 0.));
}

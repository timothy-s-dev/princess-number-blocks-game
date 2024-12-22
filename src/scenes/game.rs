use super::{despawn_screen, GameState};
use crate::animations::chest_animations::{get_chest_texture_atlas_layout, CHEST_SPRITE_SHEET};
use crate::animations::player_animations::{get_player_texture_atlas_layout, PLAYER_SPRITE_SHEET};
use crate::challenge::{
    challenge_hud_system, initialize_challenge_system, monitor_challenge_system,
};
use crate::common::components::obstacle::Obstacle;
use crate::plugins::chest::spawn_chest;
use crate::plugins::player::spawn_player;
use bevy::prelude::*;

pub fn game_plugin(app: &mut App) {
    app.add_systems(
        OnEnter(GameState::Game),
        (setup, initialize_challenge_system),
    )
    .add_systems(
        Update,
        (
            game.run_if(in_state(GameState::Game)),
            monitor_challenge_system.run_if(in_state(GameState::Game)),
            challenge_hud_system.run_if(in_state(GameState::Game)),
        ),
    )
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

    let bg_texture = asset_server.load("game_bg.png");

    commands
        .spawn(OnGameScreen)
        .insert(Transform::default())
        .insert(InheritedVisibility::default())
        .with_children(|parent| {
            parent
                .spawn(Sprite {
                    image: bg_texture.clone(),
                    image_mode: SpriteImageMode::Tiled {
                        tile_x: true,
                        tile_y: true,
                        stretch_value: 1.,
                    },
                    custom_size: Some(Vec2::new(1280., 720.)),
                    ..default()
                })
                .insert(Transform::from_xyz(0., 0., -100.));

            for x in -20..20 {
                spawn_obstacle(parent, &asset_server, 32. * x as f32 + 16., -360. + 16.);
                spawn_obstacle(parent, &asset_server, 32. * x as f32 + 16., 360. - 16.);
            }
            for y in -11..11 {
                spawn_obstacle(parent, &asset_server, -640. + 16., 32. * y as f32 + 16.);
                spawn_obstacle(parent, &asset_server, 640. - 16., 32. * y as f32 + 16.);
            }

            for chest in 0..10 {
                spawn_chest(
                    parent,
                    chest_texture.clone(),
                    chest_texture_atlas_layout.clone(),
                    -440. + (216. * (chest % 5) as f32),
                    160. - (400. * (chest / 5) as f32),
                    chest + 1,
                );
            }
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

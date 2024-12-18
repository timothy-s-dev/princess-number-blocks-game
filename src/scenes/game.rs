use super::{despawn_screen, GameState};
use crate::common_components::obstacle::Obstacle;
use crate::plugins::player::spawn_player;
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
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
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
            spawn_player(parent, asset_server, texture_atlas_layouts);
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

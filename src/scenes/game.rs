use super::{despawn_screen, GameState};
use crate::plugins::player::components::player::spawn_player;
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
            spawn_player(parent, asset_server, texture_atlas_layouts);
        });
}

// Tick the timer, and change state when finished
fn game() {
    // TODO
}

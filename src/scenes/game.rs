use bevy::prelude::*;
use super::{despawn_screen, GameState};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), game_setup)
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

fn game_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    commands.spawn((
        Sprite::from_image(asset_server.load("splash.png")),
        Transform::from_xyz(0., 0., 0.),
    ));
}

// Tick the timer, and change state when finished
fn game() {
    // TODO
}
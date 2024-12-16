use bevy::prelude::*;
use crate::plugins::player::components::player::{spawn_player};
use super::{despawn_screen, GameState};


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
    asset_server: Res<AssetServer>
) {
    commands.spawn(OnGameScreen).with_children(|parent| {
        parent.spawn((
            Sprite::from_image(asset_server.load("splash.png")),
            Transform::from_xyz(0., 0., 0.),
        ));
        spawn_player(parent);
    });
}

// Tick the timer, and change state when finished
fn game() {
    // TODO
}
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

mod scenes;
use scenes::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.30, 0.80, 0.93)))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax { max_width: 1280., max_height: 720. },
            ..OrthographicProjection::default_2d()
        },
    ));
}
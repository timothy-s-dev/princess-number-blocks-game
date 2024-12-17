use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use leafwing_input_manager::prelude::InputManagerPlugin;

pub mod animations;
mod input_map;
mod plugins;
mod scenes;

use crate::input_map::Action;
use crate::plugins::animation::animation_plugin;
use crate::plugins::player::player_plugin;
use scenes::*;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::srgb(0.30, 0.80, 0.93)))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(
            WorldInspectorPlugin::new().run_if(input_toggle_active(false, KeyCode::Backquote)),
        )
        .add_plugins(InputManagerPlugin::<Action>::default())
        // Each scene has a plugin that handles setup and teardown
        .add_plugins((splash::splash_plugin, menu::menu_plugin, game::game_plugin))
        // Other plugins (from the plugins directory) also need to be included here
        // If they should only run in certain states that should be configured in the plugin
        // See the player_plugin for an example of this
        .add_plugins((animation_plugin, player_plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::AutoMax {
                max_width: 1280.,
                max_height: 720.,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}

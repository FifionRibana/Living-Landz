// =============================================================================
// CLIENT - Main Entry Point
// =============================================================================

use bevy::asset::AssetPlugin;
use bevy::prelude::*;

mod camera;
mod input;
mod networking;
mod rendering;
mod state;
mod ui;

use camera::CameraPlugin;
use input::InputPlugin;
use networking::NetworkingPlugin;
use rendering::buildings::BuildingPlugin;
use rendering::RenderingPlugin;
use state::StatePlugin;
use ui::UiPlugin;

fn main() {
    tracing_subscriber::fmt::init();
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Living Landz".to_string(),
                        resolution: (1280, 720).into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "../../assets".to_string(),
                    ..default()
                }),
        )
        .add_plugins((
            CameraPlugin,
            NetworkingPlugin,
            StatePlugin,
            UiPlugin,
            RenderingPlugin,
            BuildingPlugin,
            InputPlugin,
        ))
        .run();
}

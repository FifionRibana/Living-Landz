// =============================================================================
// CLIENT - Main Entry Point
// =============================================================================

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
use rendering::RenderingPlugin;
use state::StatePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Living Landz".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            CameraPlugin,
            NetworkingPlugin,
            StatePlugin,
            RenderingPlugin,
            InputPlugin,
        ))
        .run();
}


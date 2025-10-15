// =============================================================================
// UI - Plugin
// =============================================================================

use bevy::prelude::*;
use super::hud::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_hud)
            .add_systems(Update, (
                update_fps_text,
                update_camera_info_text,
            ));
    }
}
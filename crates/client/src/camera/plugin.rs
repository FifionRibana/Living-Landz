// =============================================================================
// CAMERA - Plugin
// =============================================================================

use bevy::prelude::*;
pub use super::controller::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CameraSettings>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (camera_movement, camera_zoom));
    }
}
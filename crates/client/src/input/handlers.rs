// =============================================================================
// INPUT - Handlers
// =============================================================================

use crate::camera::MainCamera;
use crate::state::WorldCache;
use crate::rendering::HexConfig;
use bevy::prelude::*;
use shared::HexCoord;

pub fn mouse_pick_hex(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    hex_config: Res<HexConfig>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.single() else { return };
        let Ok((camera, camera_transform)) = camera_query.single() else { return };

        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                let hex = hex_config.layout.world_pos_to_hex(world_pos);
                let hex_coord = HexCoord::from_hex(hex);
                tracing::info!("Clicked hex: {:?} at world pos: {:?}", hex_coord, world_pos);
            }
        }
    }
}

pub fn debug_info(
    keys: Res<ButtonInput<KeyCode>>,
    cache: Res<WorldCache>,
    camera: Query<(&Transform, &Projection), With<MainCamera>>,
    hex_config: Res<HexConfig>,
) {
    if !keys.just_pressed(KeyCode::KeyI) {
        return;
    }

    let Ok((transform, projection)) = camera.single() else { return };

    let scale = if let Projection::Orthographic(ortho) = projection {
        ortho.scale
    } else {
        1.0
    };

    let hex = hex_config.layout.world_pos_to_hex(transform.translation.truncate());
    let hex_coord = HexCoord::from_hex(hex);

    tracing::info!("=== DEBUG ===");
    tracing::info!("Camera: {:?}", transform.translation);
    tracing::info!("Camera Hex: {:?}", hex_coord);
    tracing::info!("Zoom: {}", scale);
    tracing::info!("Chunks: {}", cache.chunk_count());
    tracing::info!("Hex radius: {}px", hex_config.radius());
    tracing::info!("Orientation: Flat");
}
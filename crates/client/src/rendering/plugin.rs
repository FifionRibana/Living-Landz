// =============================================================================
// RENDERING - Plugin
// =============================================================================

use bevy::prelude::*;
use super::hex_renderer::render_hex_grid;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, render_hex_grid);
    }
}
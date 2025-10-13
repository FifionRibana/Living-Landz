// =============================================================================
// RENDERING MODULE
// =============================================================================

pub mod hex_renderer;
pub mod lod;
pub mod sprites;

pub use hex_renderer::*;

use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, hex_renderer::render_hex_grid);
    }
}
// =============================================================================
// INPUT - Plugin
// =============================================================================

use bevy::prelude::*;
pub use super::handlers::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mouse_pick_hex, debug_info));
    }
}
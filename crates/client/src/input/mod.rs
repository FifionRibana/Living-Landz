// =============================================================================
// INPUT MODULE
// =============================================================================

pub mod handlers;

pub use handlers::*;

use bevy::prelude::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (mouse_pick_hex, debug_info));
    }
}
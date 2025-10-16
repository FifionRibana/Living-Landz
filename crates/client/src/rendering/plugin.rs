// =============================================================================
// RENDERING PLUGIN
// =============================================================================

use bevy::prelude::*;
use super::systems::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (
                setup_hex_config,
                setup_biome_materials,  // Après setup_hex_config
            ).chain())
            .add_systems(Update, (
                spawn_hex_sprites,
                update_hex_visuals,
                despawn_unloaded_hexes,
            ).chain());
    }
}
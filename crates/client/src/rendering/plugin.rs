// =============================================================================
// RENDERING - Plugin
// =============================================================================

use bevy::prelude::*;
use super::systems::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Setup atlas au démarrage
            .add_systems(Startup, setup_terrain_atlas)
            
            // Systèmes de rendu
            .add_systems(Update, (
                spawn_hex_sprites,
                update_hex_visuals,
                despawn_unloaded_hexes,
            ).chain());
        
        // SUPPRIMER l'ancien système render_hex_grid qui utilisait Gizmos
    }
}
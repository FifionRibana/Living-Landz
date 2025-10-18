// =============================================================================
// RENDERING PLUGIN
// =============================================================================

use super::lod::{self, SpatialGrid, ViewFrustum};
use super::systems::*;
use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            // Resources
            .insert_resource(ViewFrustum {
                center: Vec2::ZERO,
                radius: 2000.0,
            })
            .insert_resource(SpatialGrid::new(96.0))
            .insert_resource(lod::InstancedBatches::default())
            // Startup
            .add_systems(Startup, (setup_hex_config, setup_biome_materials).chain())
            .add_systems(
                Update,
                (
                    // Phase 1: Update camera & frustum
                    lod::update_view_frustum,
                    // Phase 2: Spatial indexing
                    lod::rebuild_spatial_grid,
                    // Phase 3: LOD switching
                    lod::update_lod_levels,
                    // Phase 4: Culling
                    lod::frustum_culling,
                    // Phase 5: LOD transitions (smooth)
                    lod::transition_lod_visibility,
                    // Phase 6: Instancing prep (optional, future optimization)
                    lod::prepare_instanced_batches,
                    // Phase 7: Spawning
                    spawn_hex_sprites,
                    // Phase 8: Despawning
                    despawn_unloaded_hexes,
                    // Phase 9: Visuals
                    update_hex_visuals,
                    // Debug (press G to show grid, B for batch stats)
                    lod::debug_spatial_grid,
                    lod::debug_batch_stats,
                    lod::debug_entity_count,
                )
                    .chain(),
            );
    }
}

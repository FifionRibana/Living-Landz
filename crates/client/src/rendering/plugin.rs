// =============================================================================
// RENDERING PLUGIN
// =============================================================================

use super::{buildings, systems::*};
use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_hex_config,
                setup_biome_materials,
                buildings::setup_building_materials,
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                spawn_hex_sprites,
                update_hex_visuals,
                despawn_unloaded_hexes,
                buildings::spawn_building_visuals,
                buildings::despawn_unloaded_buildings,
            )
                .chain(),
        );
    }
}

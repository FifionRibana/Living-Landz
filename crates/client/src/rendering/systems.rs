use bevy::prelude::*;
use crate::state::WorldCache;
use super::lod::LodLevel;
use super::{
    atlas::BiomeMaterials,
    components::*,
    hex_config::HexConfig,
};

/// Setup de la configuration hexagonale
pub fn setup_hex_config(mut commands: Commands) {
    let config = HexConfig::new(24.0); // Rayon = 24px
    commands.insert_resource(config);
    tracing::info!("✓ HexConfig configuré (rayon: 24.0, orientation: Flat)");
}

/// Setup des matériaux de biomes (APRÈS HexConfig)
pub fn setup_biome_materials(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    hex_config: Res<HexConfig>,
) {
    let biome_materials = BiomeMaterials::create(
        meshes,
        materials,
        hex_config,
    );
    commands.insert_resource(biome_materials);
    tracing::info!("✓ Biome materials créés");
}

/// Spawn les hexagones avec positionnement hexx
pub fn spawn_hex_sprites(
    mut commands: Commands,
    world_cache: Res<WorldCache>,
    biome_materials: Res<BiomeMaterials>,
    hex_config: Res<HexConfig>,
    existing: Query<&HexTile>,
) {
    let existing_coords: std::collections::HashSet<_> = 
        existing.iter().map(|h| h.coord).collect();
    
    for chunk in world_cache.chunks() {
        for tile_data in &chunk.tiles {
            if existing_coords.contains(&tile_data.coord) {
                continue;
            }
            
            // Conversion hexx: HexCoord → Hex → world pos
            let hex = tile_data.coord.to_hex();
            let world_pos = hex_config.layout.hex_to_world_pos(hex);
            
            let visuals = HexVisuals::new(tile_data.biome, tile_data.coord, 0);
            let material = biome_materials.get_material(tile_data.biome);
            
            commands.spawn((
                HexTile {
                    coord: tile_data.coord,
                    chunk_id: chunk.id,
                },
                visuals.clone(),
                LodLevel::Medium,
                Mesh2d(biome_materials.hex_mesh.clone()),
                MeshMaterial2d(material),
                Transform::from_translation(world_pos.extend(0.0)),
            ));
        }
    }
}

pub fn despawn_unloaded_hexes(
    mut commands: Commands,
    world_cache: Res<WorldCache>,
    hexes: Query<(Entity, &HexTile)>,
) {
    let loaded_chunks: std::collections::HashSet<_> =
        world_cache.chunks().map(|c| c.id).collect();
    
    for (entity, hex) in &hexes {
        if !loaded_chunks.contains(&hex.chunk_id) {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_hex_visuals(
    mut query: Query<(&HexVisuals, &mut MeshMaterial2d<ColorMaterial>), Changed<HexVisuals>>,
    biome_materials: Res<BiomeMaterials>,
) {
    for (visuals, mut material) in &mut query {
        let new_material = biome_materials.get_material(visuals.biome);
        material.0 = new_material;
    }
}
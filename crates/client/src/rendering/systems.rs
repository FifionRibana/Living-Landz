use bevy::prelude::*;
use crate::state::WorldCache;
use super::{atlas::TerrainAtlas, components::*, coords::hex_to_world_iso};

/// Setup initial de l'atlas
pub fn setup_terrain_atlas(
    mut commands: Commands,
    images: ResMut<Assets<Image>>,
    layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let atlas = TerrainAtlas::create_placeholder(images, layouts);
    commands.insert_resource(atlas);
    tracing::info!("✓ Terrain atlas créé");
}

/// Spawn les sprites pour les tuiles nouvellement chargées
pub fn spawn_hex_sprites(
    mut commands: Commands,
    world_cache: Res<WorldCache>,
    atlas: Res<TerrainAtlas>,
    existing: Query<&HexTile>,
) {
    // Créer un HashSet des coords déjà spawned
    let existing_coords: std::collections::HashSet<_> = 
        existing.iter().map(|h| h.coord).collect();
    
    for chunk in world_cache.chunks() {
        for tile_data in &chunk.tiles {
            // Skip si déjà spawned
            if existing_coords.contains(&tile_data.coord) {
                continue;
            }
            
            let world_pos = hex_to_world_iso(tile_data.coord);
            let texture_index = atlas.get_index(tile_data.biome);
            let visuals = HexVisuals::new(tile_data.biome, tile_data.coord);
            let tint = visuals.tint;

            commands.spawn((
                HexTile {
                    coord: tile_data.coord,
                    chunk_id: chunk.id,
                },
                visuals,
                LodLevel::Medium,
                Sprite {
                    image: atlas.texture.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: atlas.layout.clone(),
                        index: texture_index,
                    }),
                    color: tint,
                    custom_size: Some(Vec2::splat(48.0)),
                    ..default()
                },
                Transform::from_translation(world_pos.extend(0.0)),
            ));
        }
    }
}

/// Despawn les hexagones des chunks déchargés
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

/// Update visuel si le biome a changé (rare, mais utile pour futur)
pub fn update_hex_visuals(
    mut query: Query<(&HexTile, &mut HexVisuals, &mut Sprite), Changed<HexVisuals>>,
    atlas: Res<TerrainAtlas>,
) {
    for (_tile, visuals, mut sprite) in &mut query {
        let new_index = atlas.get_index(visuals.biome);
        if let Some(ref mut atlas_data) = sprite.texture_atlas {
            atlas_data.index = new_index;
        }
        sprite.color = visuals.tint;
    }
}
// =============================================================================
// CRATE HEX_GRID - lib.rs
// =============================================================================
// File: crates/hex_grid/src/lib.rs

use bevy::prelude::*;
use shared::{BiomeType, ChunkId, HexCoord, TileData};

pub const CHUNK_SIZE: u32 = 48;

#[derive(Resource, Clone)]
pub struct WorldGenerator {
    seed: u64,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    pub fn generate_chunk(&self, chunk_id: ChunkId) -> Vec<TileData> {
        let mut tiles = Vec::new();

        for local_q in 0..CHUNK_SIZE as i32 {
            for local_r in 0..CHUNK_SIZE as i32 {
                let world_q = chunk_id.x * CHUNK_SIZE as i32 + local_q;
                let world_r = chunk_id.y * CHUNK_SIZE as i32 + local_r;
                let coord = HexCoord::new(world_q, world_r);

                tiles.push(self.generate_tile(coord));
            }
        }

        tiles
    }

    fn generate_tile(&self, coord: HexCoord) -> TileData {
        // Génération simple basée sur hash pour démo
        let hash = self.hash_coord(coord);
        let biome_idx = (hash % 8) as usize;

        let biome = match biome_idx {
            0 => BiomeType::Ocean,
            1 => BiomeType::Coast,
            2 | 3 => BiomeType::Grassland,
            4 => BiomeType::Forest,
            5 => BiomeType::Mountain,
            6 => BiomeType::Desert,
            _ => BiomeType::Tundra,
        };

        TileData {
            coord,
            biome,
            altitude: ((hash % 1000) as i16) - 500,
            quality: (hash % 100) as u8,
        }
    }

    fn hash_coord(&self, coord: HexCoord) -> u64 {
        // Hash simple pour génération déterministe
        let mut hash = self.seed;
        hash ^= coord.q as u64;
        hash = hash.wrapping_mul(6364136223846793005);
        hash ^= coord.r as u64;
        hash = hash.wrapping_mul(6364136223846793005);
        hash
    }
}

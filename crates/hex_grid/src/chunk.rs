use hexx::Hex;
use serde::{Deserialize, Serialize};
use crate::biomes::BiomeType;

pub const CHUNK_SIZE: u32 = 48; // 48x48 hexagones

#[derive(Debug, Clone)]
pub struct Chunk {
    pub id: ChunkId,
    pub tiles: Vec<Tile>,
    pub dirty: bool, // Modifié depuis dernier save
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub coord: HexCoord,
    pub biome: BiomeType,
    pub altitude: i16,
    pub quality: u8,
    pub has_water: bool,
    pub has_river: bool,
}

impl Chunk {
    pub fn new(id: ChunkId, seed: u64) -> Self {
        let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);
        
        // Génération procédurale
        for local_q in 0..CHUNK_SIZE as i32 {
            for local_r in 0..CHUNK_SIZE as i32 {
                let world_q = id.x * CHUNK_SIZE as i32 + local_q;
                let world_r = id.y * CHUNK_SIZE as i32 + local_r;
                let coord = HexCoord::new(world_q, world_r);
                
                tiles.push(generate_tile(coord, seed));
            }
        }
        
        Self {
            id,
            tiles,
            dirty: false,
        }
    }
    
    pub fn get_tile(&self, coord: HexCoord) -> Option<&Tile> {
        self.tiles.iter().find(|t| t.coord == coord)
    }
}

/// Génère une tile procéduralement
fn generate_tile(coord: HexCoord, seed: u64) -> Tile {
    // Utiliser noise (ex: perlin) pour altitude, biome, etc.
    // Placeholder simple pour l'instant
    Tile {
        coord,
        biome: BiomeType::Grassland,
        altitude: 0,
        quality: 50,
        has_water: false,
        has_river: false,
    }
}
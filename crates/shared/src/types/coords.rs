use hexx::Hex;
use serde::{Deserialize, Serialize};

use super::biomes::BiomeType;

/// Coordonnées hexagonales axiales (q, r)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct HexCoord {
    pub q: i32,
    pub r: i32,
}

impl HexCoord {
    pub fn new(q: i32, r: i32) -> Self {
        Self { q, r }
    }

    pub fn to_hex(&self) -> Hex {
        Hex::new(self.q, self.r)
    }

    pub fn from_hex(hex: Hex) -> Self {
        Self { q: hex.x, r: hex.y }
    }

    /// Retourne les 6 voisins
    pub fn neighbors(&self) -> [HexCoord; 6] {
        let hex = self.to_hex();
        hex.all_neighbors()
            .map(|h| HexCoord::from_hex(h))
    }

    /// Distance Manhattan entre deux hexagones
    pub fn distance(&self, other: &HexCoord) -> u32 {
        self.to_hex().unsigned_distance_to(other.to_hex())
    }
}

/// ID d'un chunk (région de la carte)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkId {
    pub x: i32,
    pub y: i32,
}

impl ChunkId {
    pub fn from_hex_coord(coord: HexCoord, chunk_size: u32) -> Self {
        let chunk_size = chunk_size as i32;
        Self {
            x: coord.q.div_euclid(chunk_size),
            y: coord.r.div_euclid(chunk_size),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileData {
    pub coord: HexCoord,
    pub biome: BiomeType,
    pub altitude: i16,
    pub quality: u8,
}
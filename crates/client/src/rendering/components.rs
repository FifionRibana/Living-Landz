use bevy::prelude::*;
use shared::{BiomeType, HexCoord, ChunkId};

#[derive(Component)]
pub struct HexTile {
    pub coord: HexCoord,
    pub chunk_id: ChunkId,
}

#[derive(Component, Clone)]
pub struct HexVisuals {
    pub biome: BiomeType,
    pub tint: Color,
    pub texture_index: usize
}

impl HexVisuals {
    pub fn new(biome: BiomeType, coord: HexCoord, texture_index: usize) -> Self {
        // Variation de couleur basée sur coord
        let seed = (coord.q as u64).wrapping_mul(374761393)
            ^ (coord.r as u64).wrapping_mul(668265263);
        let variation = ((seed % 20) as f32 - 10.0) / 100.0; // ±10%
        
        let tint = Color::srgb(
            (1.0 + variation).clamp(0.0, 1.0),
            (1.0 + variation).clamp(0.0, 1.0),
            (1.0 + variation).clamp(0.0, 1.0),
        );
        
        Self {
            biome,
            tint,
            texture_index
        }
    }
}
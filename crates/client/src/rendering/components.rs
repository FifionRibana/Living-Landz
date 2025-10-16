use bevy::prelude::*;
use shared::{BiomeType, HexCoord, ChunkId};

/// Marqueur pour une tuile hexagonale
#[derive(Component)]
pub struct HexTile {
    pub coord: HexCoord,
    pub chunk_id: ChunkId,
}

/// Niveau de détail (pour Phase 1: Medium uniquement)
#[derive(Component, Clone, Copy, PartialEq)]
pub enum LodLevel {
    Medium, // Seul niveau pour Phase 1
}

/// Données visuelles d'un hexagone
#[derive(Component, Clone)]
pub struct HexVisuals {
    pub biome: BiomeType,
    pub texture_index: usize,
    pub tint: Color, // Variation légère de couleur
}

impl HexVisuals {
    pub fn new(biome: BiomeType, coord: HexCoord) -> Self {
        // Variation de couleur basée sur coord (déterministe)
        let seed = (coord.q as u64).wrapping_mul(374761393)
            ^ (coord.r as u64).wrapping_mul(668265263);
        let variation = (seed % 20) as f32 / 100.0; // ±10% variation
        
        let tint = Color::srgb(
            0.95 + variation,
            0.95 + variation,
            0.95 + variation,
        );
        
        Self {
            biome,
            texture_index: 0, // Sera assigné par l'atlas
            tint,
        }
    }
}
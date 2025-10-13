// =============================================================================
// WORLD COMPONENTS
// =============================================================================

use serde::{Deserialize, Serialize};
use shared::types::*;
use shared::types::resources::*;

// ============================================================================
// TERRAIN
// ============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HexTile {
    pub coord: HexCoord,
    pub biome: BiomeType,
    pub altitude: i16,
    pub quality: u8,
    pub is_river: bool,
    pub river_flow: Option<u8>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ChunkCoord {
    pub x: i32,
    pub y: i32,
}

impl ChunkCoord {
    pub fn from_hex(hex: HexCoord, chunk_size: u32) -> Self {
        Self {
            x: hex.q.div_euclid(chunk_size as i32),
            y: hex.r.div_euclid(chunk_size as i32),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chunk {
    pub coord: ChunkCoord,
    pub tiles: Vec<HexTile>,
    pub generated_at: u64,
}

// ============================================================================
// ENTITIES (existant)
// ============================================================================

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub coord: HexCoord,
}

#[derive(Clone, Debug)]
pub struct Building {
    pub building_type: BuildingType,
    pub owner_id: u64,
    pub construction_progress: f32,
    pub health: u8,
}

#[derive(Clone, Debug)]
pub struct Unit {
    pub name: String,
    pub stats: Stats,
    pub profession: Option<Profession>,
    pub inventory: Vec<Resource>,
    pub home_city_id: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub strength: u8,
    pub agility: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

#[derive(Clone, Debug)]
pub struct City {
    pub name: String,
    pub owner_id: u64,
    pub population: u32,
    pub inventory: std::collections::HashMap<ResourceType, Vec<Resource>>,
}

#[derive(Clone, Copy, Debug)]
pub struct NetworkedEntity;
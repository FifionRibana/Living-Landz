// =============================================================================
// STATE - World Cache
// =============================================================================

use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use shared::{ChunkId, TileData, HexCoord};

#[derive(Resource, Default)]
pub struct WorldCache {
    chunks: HashMap<ChunkId, ChunkData>,
    requested: HashSet<ChunkId>,
}

#[derive(Clone)]
pub struct ChunkData {
    pub id: ChunkId,
    pub tiles: Vec<TileData>,
    pub loaded_at: f32,
}

impl WorldCache {
    pub fn insert_chunk(&mut self, id: ChunkId, tiles: Vec<TileData>, time: f32) {
        self.chunks.insert(id, ChunkData {
            id,
            tiles,
            loaded_at: time,
        });
        self.requested.remove(&id);
    }
    
    pub fn get_chunk(&self, id: &ChunkId) -> Option<&ChunkData> {
        self.chunks.get(id)
    }
    
    pub fn get_tile(&self, coord: HexCoord) -> Option<&TileData> {
        let chunk_id = coord_to_chunk(coord);
        self.chunks.get(&chunk_id)?.tiles.iter().find(|t| t.coord == coord)
    }
    
    pub fn is_loaded(&self, id: &ChunkId) -> bool {
        self.chunks.contains_key(id)
    }
    
    pub fn is_requested(&self, id: &ChunkId) -> bool {
        self.requested.contains(id)
    }
    
    pub fn mark_requested(&mut self, id: ChunkId) {
        self.requested.insert(id);
    }
    
    pub fn chunks(&self) -> impl Iterator<Item = &ChunkData> {
        self.chunks.values()
    }
    
    pub fn unload_distant(&mut self, center: ChunkId, max_distance: i32) {
        self.chunks.retain(|id, _| {
            (id.x - center.x).abs() <= max_distance &&
            (id.y - center.y).abs() <= max_distance
        });
    }
    
    pub fn chunk_count(&self) -> usize {
        self.chunks.len()
    }
}

const CHUNK_SIZE: i32 = 60;

fn coord_to_chunk(coord: HexCoord) -> ChunkId {
    ChunkId {
        x: coord.q.div_euclid(CHUNK_SIZE),
        y: coord.r.div_euclid(CHUNK_SIZE),
    }
}
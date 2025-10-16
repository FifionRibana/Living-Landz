use bevy::prelude::*;
use shared::{HexCoord, ChunkId};

/// Convertit coordonnées hex en position monde (isométrique)
pub fn hex_to_world_iso(coord: HexCoord) -> Vec2 {
    const HEX_WIDTH: f32 = 48.0;
    const HEX_HEIGHT: f32 = 48.0 * 0.866;
    
    Vec2::new(
        coord.q as f32 * HEX_WIDTH + coord.r as f32 * HEX_WIDTH * 0.5,
        coord.r as f32 * HEX_HEIGHT,
    )
}

/// Convertit position monde en coordonnées hex approximatives
pub fn world_to_hex(pos: Vec2) -> HexCoord {
    const HEX_WIDTH: f32 = 48.0;
    const HEX_HEIGHT: f32 = 48.0 * 0.866;
    
    let q = ((pos.x / HEX_WIDTH) - 0.5 * (pos.y / HEX_HEIGHT)) as i32;
    let r = (pos.y / HEX_HEIGHT) as i32;
    
    HexCoord::new(q, r)
}

/// Convertit position monde en ChunkId
pub fn world_pos_to_chunk(pos: Vec2) -> ChunkId {
    const CHUNK_SIZE: i32 = 60;
    let hex = world_to_hex(pos);
    
    ChunkId {
        x: hex.q.div_euclid(CHUNK_SIZE),
        y: hex.r.div_euclid(CHUNK_SIZE),
    }
}
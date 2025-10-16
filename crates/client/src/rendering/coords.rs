use bevy::prelude::*;
use hexx::{Hex, HexLayout};
use shared::{HexCoord, ChunkId};

/// Convertit HexCoord (shared) en Hex (hexx)
pub fn hexcoord_to_hex(coord: HexCoord) -> Hex {
    Hex::new(coord.q, coord.r)
}

/// Convertit Hex (hexx) en HexCoord (shared)
pub fn hex_to_hexcoord(hex: Hex) -> HexCoord {
    HexCoord::new(hex.x, hex.y)
}

/// Convertit coordonnées hexagonales en position monde (pixel)
/// Utilise le HexLayout global
pub fn hex_to_world(coord: HexCoord, layout: &HexLayout) -> Vec2 {
    let hex = hexcoord_to_hex(coord);
    layout.hex_to_world_pos(hex)
}

/// Convertit position monde (pixel) en coordonnées hexagonales
pub fn world_to_hex(pos: Vec2, layout: &HexLayout) -> HexCoord {
    let hex = layout.world_pos_to_hex(pos);
    hex_to_hexcoord(hex)
}

/// Convertit position monde en ChunkId
pub fn world_pos_to_chunk(pos: Vec2, layout: &hexx::HexLayout) -> ChunkId {
    const CHUNK_SIZE: i32 = 60;
    let hex = layout.world_pos_to_hex(pos);
    let hex_coord = HexCoord::from_hex(hex);
    
    ChunkId {
        x: hex_coord.q.div_euclid(CHUNK_SIZE),
        y: hex_coord.r.div_euclid(CHUNK_SIZE),
    }
}
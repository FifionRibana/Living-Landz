// =============================================================================
// CRATE SHARED - protocol/mod.rs
// =============================================================================
// File: crates/shared/src/protocol/mod.rs

use serde::{Deserialize, Serialize};
use crate::types::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    Login {
        username: String,
    },
    RequestChunks {
        chunk_ids: Vec<ChunkId>,
    },
    BuildAction {
        coord: HexCoord,
        building_type: BuildingType,
    },
    Ping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    LoginSuccess {
        player_id: u64,
    },
    ChunkData {
        chunk_id: ChunkId,
        tiles: Vec<TileData>,
    },
    WorldTick {
        tick: u64,
    },
    Pong,
}
use serde::{Deserialize, Serialize};
use crate::types::*;

/// Messages Client → Serveur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Connexion initiale
    Login {
        username: String,
        password_hash: String,
    },
    
    /// Demande chunks autour d'une position
    RequestChunks {
        chunk_ids: Vec<ChunkId>,
    },
    
    /// Action de construction
    Build {
        coord: HexCoord,
        building_type: BuildingType,
    },
    
    /// Déplacement d'unité
    MoveUnit {
        unit_id: u64,
        destination: HexCoord,
    },
    
    /// Ping (keepalive)
    Ping,
}

/// Messages Serveur → Client
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    /// Confirmation de connexion
    LoginSuccess {
        player_id: u64,
        character_id: u64,
    },
    
    /// Erreur de connexion
    LoginError {
        reason: String,
    },
    
    /// État global du monde (tick)
    WorldState {
        tick: u64,
        timestamp: u64,
    },
    
    /// Données d'un chunk
    ChunkData {
        chunk_id: ChunkId,
        tiles: Vec<TileData>,
        entities: Vec<EntitySnapshot>,
    },
    
    /// Notification de tick
    TickNotification {
        next_tick_in_seconds: u64,
    },
    
    /// Résultat d'une action
    ActionResult {
        success: bool,
        message: String,
    },
    
    /// Pong (réponse au ping)
    Pong,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TileData {
    pub coord: HexCoord,
    pub biome: BiomeType,
    pub altitude: i16,
    pub quality: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntitySnapshot {
    pub id: u64,
    pub entity_type: EntityType,
    pub position: HexCoord,
    // ... autres données selon type
}
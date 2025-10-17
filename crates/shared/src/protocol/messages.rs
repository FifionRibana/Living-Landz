use serde::{Deserialize, Serialize};
use crate::types::*;

/// Messages Client → Serveur
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    /// Connexion initiale
    Login {
        username: String,
        // password_hash: String,
    },
    
    /// Demande chunks autour d'une position
    RequestChunks {
        chunk_ids: Vec<ChunkId>,
    },
    
    /// Action de construction
    BuildAction {
        coord: HexCoord,
        building_type: BuildingType,
    },
    
    /// Déplacement d'unité
    MoveUnit {
        unit_id: u64,
        destination: HexCoord,
    },
    
    /// Action sur bâtiment
    BuildingAction {
        building_id: u64,
        action: BuildingActionType,
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
    },
    
    /// Erreur de connexion
    LoginError {
        reason: String,
    },

    WorldTick {
        tick: u64,
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
        buildings: Vec<BuildingData>,
        // entities: Vec<EntitySnapshot>,
    },
    
    /// Update bâtiment individuel
    BuildingUpdate {
        building: Building,
        tree_data: Option<TreeData>,
    },
    
    /// Destruction bâtiment
    BuildingDestroyed {
        building_id: u64,
        coord: HexCoord,
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

// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub struct EntitySnapshot {
//     pub id: u64,
//     pub entity_type: EntityType,
//     pub position: HexCoord,
//     // ... autres données selon type
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildingData {
    pub building: Building,
    pub tree_data: Option<TreeData>,
}
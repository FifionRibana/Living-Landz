// =============================================================================
// STATE - Chunk Streaming Logic
// =============================================================================

use bevy::prelude::*;
use shared::{ChunkId, ServerMessage};
use crate::networking::NetworkClient;
use super::{world_cache::WorldCache, connection::ConnectionStatus};
use tracing::*;

#[derive(Resource)]
pub struct StreamingConfig {
    pub view_radius: i32,
    pub unload_distance: i32,
    pub request_cooldown: f32,
    last_request: f32,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        Self {
            view_radius: 3,
            unload_distance: 5,
            request_cooldown: 0.5, // 500ms entre requêtes
            last_request: -999.0,
        }
    }
}

pub fn request_chunks_around_camera(
    camera: Query<&Transform, With<Camera2d>>,
    mut cache: ResMut<WorldCache>,
    mut config: ResMut<StreamingConfig>,
    network: Option<ResMut<NetworkClient>>,
    connection: Res<ConnectionStatus>,
    time: Res<Time>,
) {
    let Some(mut net) = network else { return };
    let Ok(transform) = camera.single() else { return };

    // Wait for login confirmation
    if !connection.is_ready() {
        return;
    }
    
    // Throttle requests
    if time.elapsed_secs() - config.last_request < config.request_cooldown {
        return;
    }

    // tracing::info!("Pos: {:?}", transform.translation.truncate());
    let center_chunk = world_pos_to_chunk(transform.translation.truncate());
    let mut to_request = Vec::new();
    // tracing::info!("Chunk: ({}, {})", center_chunk.x, center_chunk.y);

    for dx in -config.view_radius..=config.view_radius {
        for dy in -config.view_radius..=config.view_radius {
            let chunk_id = ChunkId {
                x: center_chunk.x + dx,
                y: center_chunk.y + dy,
            };
            
            if !cache.is_loaded(&chunk_id) && !cache.is_requested(&chunk_id) {
                cache.mark_requested(chunk_id);
                to_request.push(chunk_id);
            }
        }
    }
    
    if !to_request.is_empty() {
        tracing::info!("Requesting {} chunks", to_request.len());
        net.send_message(shared::ClientMessage::RequestChunks {
            chunk_ids: to_request,
        });
        config.last_request = time.elapsed_secs();
        tracing::info!("Sent request for chunks");
    }
}

pub fn process_chunk_messages(
    mut cache: ResMut<WorldCache>,
    mut connection: ResMut<ConnectionStatus>,
    network: Option<ResMut<NetworkClient>>,
    time: Res<Time>,
) {
    let Some(mut net) = network else { return };

    let messages = net.poll_messages();
    if !messages.is_empty() {
        tracing::info!("Received {} messages from server", messages.len());
    }
    
    for msg in messages {
        match msg {
            ServerMessage::LoginSuccess { player_id } => {
                tracing::info!("✓ Login successful, player ID: {}", player_id);
                connection.logged_in = true;
                connection.player_id = Some(player_id);
            }
            ServerMessage::ChunkData { chunk_id, tiles } => {
                tracing::info!("✓ Received chunk ({}, {}) with {} tiles", chunk_id.x, chunk_id.y, tiles.len());
                cache.insert_chunk(chunk_id, tiles, time.elapsed_secs());
                tracing::info!("✓ Loaded chunk {:?}", chunk_id);
            }
            _ => {
                tracing::warn!("Unhandled server message: {:?}", msg);
            }
        }
    }
}

pub fn unload_distant_chunks(
    camera: Query<&Transform, With<Camera2d>>,
    mut cache: ResMut<WorldCache>,
    config: Res<StreamingConfig>,
) {
    let Ok(transform) = camera.single() else { return };
    let center = world_pos_to_chunk(transform.translation.truncate());
    cache.unload_distant(center, config.unload_distance);
}

const CHUNK_SIZE: i32 = 60;
const HEX_SIZE: f32 = 20.0;

fn world_pos_to_chunk(pos: Vec2) -> ChunkId {
    let sqrt3 = 1.732050808;
    let q = ((pos.x * sqrt3/3.0 - pos.y / 3.0) / HEX_SIZE) as i32;
    let r = ((pos.y * 2.0/3.0) / HEX_SIZE) as i32;
    
    ChunkId {
        x: q.div_euclid(CHUNK_SIZE),
        y: r.div_euclid(CHUNK_SIZE),
    }
}
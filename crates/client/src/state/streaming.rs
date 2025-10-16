// =============================================================================
// STATE - Chunk Streaming Logic
// =============================================================================

use bevy::prelude::*;
use shared::{ChunkId, ServerMessage};
use crate::networking::NetworkClient;
use crate::rendering::{HexConfig, coords::world_pos_to_chunk};
use super::{world_cache::WorldCache, connection::ConnectionStatus};

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
            request_cooldown: 0.5,
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
    hex_config: Res<HexConfig>,
    time: Res<Time>,
) {
    let Some(mut net) = network else { return };
    let Ok(transform) = camera.single() else { return };

    if !connection.is_ready() {
        return;
    }
    
    if time.elapsed_secs() - config.last_request < config.request_cooldown {
        return;
    }

    let center_chunk = world_pos_to_chunk(
        transform.translation.truncate(),
        &hex_config.layout
    );
    let mut to_request = Vec::new();

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
                tracing::info!("✓ Received chunk ({}, {}) with {} tiles",
                    chunk_id.x, chunk_id.y, tiles.len());
                cache.insert_chunk(chunk_id, tiles, time.elapsed_secs());
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
    hex_config: Res<HexConfig>,
) {
    let Ok(transform) = camera.single() else { return };
    let center = world_pos_to_chunk(
        transform.translation.truncate(),
        &hex_config.layout
    );
    cache.unload_distant(center, config.unload_distance);
}
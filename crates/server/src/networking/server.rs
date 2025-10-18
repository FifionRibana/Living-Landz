// =============================================================================
// NETWORKING - Server
// =============================================================================

use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::{accept_async, tungstenite::Message};

use hex_grid::WorldGenerator;
use shared::{ClientMessage, ServerMessage, TileData};

use super::Sessions;
use crate::{database::BuildingDatabase, database::ChunkDatabase, world::ChunkCoord};

// use rand::prelude::*;
// use bevy::prelude::*;

pub struct NetworkServer {
    pub address: String,
    pub port: u16,
}

impl NetworkServer {
    pub fn new(address: String, port: u16) -> Self {
        Self { address, port }
    }

    pub async fn start(
        &self,
        sessions: Sessions,
        world_gen: WorldGenerator,
        chunk_db: Arc<ChunkDatabase>,
        building_db: Arc<BuildingDatabase>,
    ) {
        let addr = format!("{}:{}", self.address, self.port);
        let listener = TcpListener::bind(&addr)
            .await
            .expect("Failed to bind server");

        tracing::info!("🌐 Server listening on {}", addr);

        while let Ok((stream, addr)) = listener.accept().await {
            let sessions_clone = sessions.clone();
            let world_gen_clone = world_gen.clone();
            let chunk_db_clone = chunk_db.clone();
            let building_db_clone = building_db.clone();

            tokio::spawn(async move {
                handle_connection(
                    stream,
                    addr,
                    sessions_clone,
                    world_gen_clone,
                    chunk_db_clone,
                    building_db_clone,
                )
                .await;
            });
        }
    }
}

pub fn initialize_server(
    sessions: Sessions,
    world_gen: WorldGenerator,
    chunk_db: Arc<ChunkDatabase>,
    building_db: Arc<BuildingDatabase>,
) {
    tracing::info!("Starting network server...");

    // Normal server startup
    let server_address =
        std::env::var("SERVER_ADDRESS").unwrap_or_else(|_| "127.0.0.1".to_string());
    let server_port: u16 = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "9001".to_string())
        .parse()
        .unwrap_or(9001);

    let sessions_clone = sessions.clone();
    let world_gen_clone = world_gen.clone();
    let chunk_db_clone = chunk_db.clone();
    let building_db_clone = building_db.clone();

    tokio::spawn(async move {
        let server = NetworkServer::new(server_address, server_port);
        server
            .start(
                sessions_clone,
                world_gen_clone,
                chunk_db_clone,
                building_db_clone,
            )
            .await;
    });

    tracing::info!("✓ Network server spawned");
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    sessions: Sessions,
    world_gen: WorldGenerator,
    chunk_db: Arc<ChunkDatabase>,
    building_db: Arc<BuildingDatabase>,
) {
    tracing::info!("New connection from {}", addr);

    let ws_stream = match accept_async(stream).await {
        Ok(ws) => ws,
        Err(e) => {
            tracing::error!("WebSocket handshake error: {}", e);
            return;
        }
    };

    let (mut write, mut read) = ws_stream.split();
    let player_id = rand::random::<u64>();

    sessions.insert(player_id, addr);

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                tracing::info!("Received message from {}: {} bytes", addr, data.len());
                if let Ok(client_msg) = bincode::deserialize::<ClientMessage>(&data) {
                    tracing::debug!("Received: {:?}", client_msg);

                    let responses = handle_client_message(
                        client_msg,
                        player_id,
                        &world_gen,
                        &chunk_db,
                        &building_db,
                    )
                    .await;

                    for response in responses {
                        if let Ok(response_data) = bincode::serialize(&response) {
                            let _ = write.send(Message::Binary(response_data)).await;
                        }
                    }
                } else {
                    tracing::warn!("Failed to deserialize message from {}", addr);
                }
            }
            Ok(Message::Close(_)) => break,
            Err(e) => {
                tracing::error!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    sessions.remove(&player_id).await;
    tracing::info!("Connection closed: {}", addr);
}

async fn handle_client_message(
    msg: ClientMessage,
    player_id: u64,
    world_gen: &WorldGenerator,
    chunk_db: &ChunkDatabase,
    building_db: &BuildingDatabase,
) -> Vec<ServerMessage> {
    match msg {
        ClientMessage::Login { username } => {
            tracing::info!("Player {} logged in as {}", player_id, username);
            vec![ServerMessage::LoginSuccess { player_id }]
        }
        ClientMessage::RequestChunks { chunk_ids } => {
            // Try loading from DB first
            tracing::info!("Player {} requested chunks: {:?}", player_id, chunk_ids);

            let mut responses = Vec::new();
            for chunk_id in chunk_ids {
                tracing::debug!("Request chunk ({}, {})", chunk_id.x, chunk_id.y);
                let chunk_coord = ChunkCoord {
                    x: chunk_id.x,
                    y: chunk_id.y,
                };

                match chunk_db.load_chunk(chunk_coord).await {
                    Ok(Some(chunk)) => {
                        let buildings = match building_db.load_buildings(chunk_coord).await {
                            Ok(b) => b,
                            Err(e) => {
                                tracing::error!("Failed to load buildings: {}", e);
                                Vec::new()
                            }
                        };
                        // Return from DB
                        tracing::debug!(
                            "Loaded chunk ({}, {}) from DB with {} buildings",
                            chunk_id.x,
                            chunk_id.y,
                            buildings.len()
                        );

                        responses.push(ServerMessage::ChunkData {
                            chunk_id,
                            tiles: chunk
                                .tiles
                                .iter()
                                .map(|t| TileData {
                                    coord: t.coord,
                                    biome: t.biome,
                                    altitude: t.altitude,
                                    quality: t.quality,
                                })
                                .collect(),
                            buildings,
                        });
                    }
                    Ok(None) => {
                        tracing::warn!(
                            "Chunk ({}, {}) not in DB, generating",
                            chunk_id.x,
                            chunk_id.y
                        );

                        let tiles = world_gen.generate_chunk(chunk_id);
                        responses.push(ServerMessage::ChunkData {
                            chunk_id,
                            tiles,
                            buildings: Vec::new(), // Pas de génération via world_gen simplifié
                        });
                    }
                    Err(e) => {
                        tracing::error!(
                            "DB error for chunk ({}, {}): {}",
                            chunk_id.x,
                            chunk_id.y,
                            e
                        );
                        let tiles = world_gen.generate_chunk(chunk_id);
                        responses.push(ServerMessage::ChunkData {
                            chunk_id,
                            tiles,
                            buildings: Vec::new(),
                        });
                    }
                }
            }

            tracing::debug!("Sending {} chunk responses", responses.len());

            responses
        }
        ClientMessage::Ping => vec![ServerMessage::Pong],
        _ => vec![ServerMessage::Pong],
    }
}

async fn broadcast_message(sessions: Sessions, msg: ServerMessage) {
    let count = sessions.count().await;
    tracing::debug!("Broadcasting message to {} sessions: {:?}", count, msg);
    // TODO: implement proper broadcasting
}

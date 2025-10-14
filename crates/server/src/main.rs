// =============================================================================
// CRATE SERVER - main.rs
// =============================================================================

use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use rand::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::Error, tungstenite::Message};

use hex_grid::WorldGenerator;
use shared::{ChunkId, ClientMessage, ServerMessage};

mod world;
use world::*;

type Sessions = Arc<RwLock<HashMap<u64, SocketAddr>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    // Check if world generation needed
    let args: Vec<String> = std::env::args().collect();
    if args.contains(&"--generate-world".to_string()) {
        tracing::info!("=== Starting World Generation ===");
        world::systems::generate_world_complete().await;
        tracing::info!("=== Generation Complete - Exiting ===");
        return;
    }

    // Normal server startup
    let addr = "127.0.0.1:9001".to_string();
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server listening on: {}", addr);

    let sessions: Sessions = Arc::new(RwLock::new(HashMap::new()));
    let world_gen = Arc::new(WorldGenerator::new(12345));

    // Initialize database connection
    let db = initialize_database().await;
    let db = Arc::new(db);

    // Tick system
    let sessions_clone = sessions.clone();
    tokio::spawn(async move {
        let mut tick: u64 = 0;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            tick += 1;
            tracing::info!("Tick {}", tick);

            let msg = ServerMessage::WorldTick { tick };
            broadcast_message(&sessions_clone, msg).await;
        }
    });

    // Accept connections
    while let Ok((stream, addr)) = listener.accept().await {
        let sessions = sessions.clone();
        let world_gen = world_gen.clone();
        let db = db.clone();
        tokio::spawn(handle_connection(stream, addr, sessions, world_gen, db));
    }
}

async fn initialize_database() -> ChunkDatabase {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost/living_landz".to_string());
    
    tracing::info!("Connecting to database at {}", database_url);
    
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");
    
    let db = ChunkDatabase::new(pool);
    db.init_schema().await.expect("Failed to init schema");
    
    tracing::info!("✓ Database connected");
    db
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    sessions: Sessions,
    world_gen: Arc<WorldGenerator>,
    db: Arc<ChunkDatabase>,
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

    sessions.write().await.insert(player_id, addr);

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                tracing::info!("Received message from {}: {} bytes", addr, data.len());
                if let Ok(client_msg) = bincode::deserialize::<ClientMessage>(&data) {
                    tracing::info!("ClientMessage: {:?}", client_msg);
                    let response = handle_client_message(
                        client_msg, 
                        player_id, 
                        &world_gen,
                        &db,
                    ).await;

                    if let Ok(response_data) = bincode::serialize(&response) {
                        let _ = write.send(Message::Binary(response_data)).await;
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

    sessions.write().await.remove(&player_id);
    tracing::info!("Connection closed: {}", addr);
}

async fn handle_client_message(
    msg: ClientMessage,
    player_id: u64,
    world_gen: &WorldGenerator,
    db: &ChunkDatabase,
) -> ServerMessage {
    match msg {
        ClientMessage::Login { username } => {
            tracing::info!("Player {} logged in as {}", player_id, username);
            ServerMessage::LoginSuccess { player_id }
        }
        ClientMessage::RequestChunks { chunk_ids } => {
            // Try loading from DB first
            tracing::info!("Player {} requested chunks: {:?}", player_id, chunk_ids);
            if let Some(&chunk_id) = chunk_ids.first() {
                let chunk_coord = ChunkCoord {
                    x: chunk_id.x,
                    y: chunk_id.y,
                };
                
                match db.load_chunk(chunk_coord).await {
                    Ok(Some(chunk)) => {
                        // Return from DB
                        return ServerMessage::ChunkData {
                            chunk_id,
                            tiles: chunk.tiles.iter().map(|t| shared::TileData {
                                coord: t.coord,
                                biome: t.biome,
                                altitude: t.altitude,
                                quality: t.quality,
                            }).collect(),
                        };
                    }
                    _ => {
                        // Fallback: generate on-the-fly
                        let tiles = world_gen.generate_chunk(chunk_id);
                        return ServerMessage::ChunkData { chunk_id, tiles };
                    }
                }
            }
            ServerMessage::Pong
        }
        ClientMessage::Ping => ServerMessage::Pong,
        _ => ServerMessage::Pong,
    }
}

async fn broadcast_message(sessions: &Sessions, msg: ServerMessage) {
    // TODO: implement proper broadcasting
}
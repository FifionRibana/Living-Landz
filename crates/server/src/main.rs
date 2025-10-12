// =============================================================================
// CRATE SERVER - main.rs
// =============================================================================
// File: crates/server/src/main.rs

use futures::{SinkExt, StreamExt};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use rand::prelude::*;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::RwLock;
use tokio_tungstenite::{accept_async, tungstenite::Error, tungstenite::Message};
// use tungstenite::protocol::Message;

use hex_grid::WorldGenerator;
use shared::{ChunkId, ClientMessage, ServerMessage};

type Sessions = Arc<RwLock<HashMap<u64, SocketAddr>>>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let addr = "127.0.0.1:9001".to_string();
    let listener = TcpListener::bind(&addr).await.unwrap();
    tracing::info!("Server listening on: {}", addr);

    let sessions: Sessions = Arc::new(RwLock::new(HashMap::new()));
    let world_gen = Arc::new(WorldGenerator::new(12345));

    // Tick system (separate task)
    let sessions_clone = sessions.clone();
    tokio::spawn(async move {
        let mut tick: u64 = 0;
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
            tick += 1;
            tracing::info!("Tick {}", tick);

            // Broadcast tick à tous les clients
            let msg = ServerMessage::WorldTick { tick };
            broadcast_message(&sessions_clone, msg).await;
        }
    });

    // Accept connections
    while let Ok((stream, addr)) = listener.accept().await {
        let sessions = sessions.clone();
        let world_gen = world_gen.clone();
        tokio::spawn(handle_connection(stream, addr, sessions, world_gen));
    }
}

async fn handle_connection(
    stream: TcpStream,
    addr: SocketAddr,
    sessions: Sessions,
    world_gen: Arc<WorldGenerator>,
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

    // Enregistrer session
    sessions.write().await.insert(player_id, addr);

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Binary(data)) => {
                if let Ok(client_msg) = bincode::deserialize::<ClientMessage>(&data) {
                    let response = handle_client_message(client_msg, player_id, &world_gen);

                    if let Ok(response_data) = bincode::serialize(&response) {
                        let _ = write.send(Message::Binary(response_data)).await;
                    }
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

    // Cleanup
    sessions.write().await.remove(&player_id);
    tracing::info!("Connection closed: {}", addr);
}

fn handle_client_message(
    msg: ClientMessage,
    player_id: u64,
    world_gen: &WorldGenerator,
) -> ServerMessage {
    match msg {
        ClientMessage::Login { username } => {
            tracing::info!("Player {} logged in as {}", player_id, username);
            ServerMessage::LoginSuccess { player_id }
        }
        ClientMessage::RequestChunks { chunk_ids } => {
            // Pour démo, on renvoie le premier chunk
            if let Some(&chunk_id) = chunk_ids.first() {
                let tiles = world_gen.generate_chunk(chunk_id);
                ServerMessage::ChunkData { chunk_id, tiles }
            } else {
                ServerMessage::Pong // Fallback
            }
        }
        ClientMessage::Ping => ServerMessage::Pong,
        _ => ServerMessage::Pong,
    }
}

async fn broadcast_message(sessions: &Sessions, msg: ServerMessage) {
    // TODO: implement proper broadcasting
}

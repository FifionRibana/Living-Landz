// =============================================================================
// NETWORKING - Client
// =============================================================================

use bevy::prelude::*;
use shared::{ClientMessage, ServerMessage};
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use tungstenite::Message;

#[derive(Resource)]
pub struct NetworkClient {
    outgoing: Sender<Vec<u8>>,
    incoming: Arc<Mutex<VecDeque<ServerMessage>>>,
    connected: Arc<Mutex<bool>>,
}

impl NetworkClient {
    pub fn connect(server_url: &str) -> Result<Self, String> {
        tracing::info!("Connecting to {}", server_url);

        // Parse URL to extract host:port
        let url = url::Url::parse(server_url).map_err(|e| format!("Invalid URL: {}", e))?;
        let host = url.host_str().ok_or("No host in URL")?;
        let port = url.port().unwrap_or(9001);

        // Connect TCP directly
        let tcp_stream = std::net::TcpStream::connect(format!("{}:{}", host, port))
            .map_err(|e| format!("TCP connection failed: {}", e))?;

        tracing::info!("TCP connected, upgrading to WebSocket...");

        let (mut socket, _) = tungstenite::client(server_url, tcp_stream)
            .map_err(|e| format!("Connection failed: {}", e))?;

        // Set non-blocking BEFORE WebSocket upgrade
        socket
            .get_mut()
            .set_nonblocking(true)
            .map_err(|e| format!("Failed to set non-blocking: {}", e))?;

        let (tx, rx) = channel::<Vec<u8>>();
        let incoming = Arc::new(Mutex::new(VecDeque::new()));
        let connected = Arc::new(Mutex::new(true));

        // Spawn a thread to read messages
        let incoming_clone = incoming.clone();
        let connected_clone = connected.clone();

        thread::spawn(move || {
            loop {
                // Check if disconnected
                if !*connected_clone.lock().unwrap() {
                    break;
                }

                let mut sent_any = false;

                while let Ok(data) = rx.try_recv() {
                    tracing::info!("Sending {} bytes to server", data.len());
                    if let Err(e) = socket.write(Message::Binary(data.into())) {
                        tracing::error!("Write error: {}", e);
                        *connected_clone.lock().unwrap() = false;
                        return;
                    } else {
                        tracing::info!("✓ Message sent");
                    }
                    sent_any = true;
                }

                // Flush immediately
                if sent_any {
                    if let Err(e) = socket.flush() {
                        tracing::error!("Flush error: {}", e);
                        *connected_clone.lock().unwrap() = false;
                        return;
                    }
                    tracing::info!("✓ Messages flushed");
                }

                match socket.can_read() {
                    true => {
                        match socket.read() {
                            Ok(Message::Binary(data)) => {
                                tracing::info!("Received {} bytes from server", data.len());
                                match bincode::deserialize::<ServerMessage>(&data) {
                                    Ok(server_msg) => {
                                        // tracing::info!("✓ Deserialized ServerMessage: {:?}", server_msg);
                                        incoming_clone.lock().unwrap().push_back(server_msg);
                                    }
                                    Err(e) => {
                                        tracing::error!("Deserialize error: {}", e);
                                    }
                                }
                            }
                            Ok(Message::Close(_)) => {
                                tracing::info!("Server closed connection");
                                *connected_clone.lock().unwrap() = false;
                                break;
                            }
                            Ok(_) => {}
                            Err(tungstenite::Error::Io(ref e))
                                if e.kind() == std::io::ErrorKind::WouldBlock =>
                            {
                                // No-blocking read, no data available
                                thread::sleep(std::time::Duration::from_millis(10));
                            }
                            Err(e) => {
                                tracing::error!("Error reading: {}", e);
                                thread::sleep(std::time::Duration::from_millis(10));
                            }
                        }
                    }
                    false => {
                        // No data available
                        thread::sleep(std::time::Duration::from_millis(10));
                    }
                }
            }

            tracing::info!("Exiting network thread");
        });

        tracing::info!("✓ Connected to server");

        Ok(Self {
            outgoing: tx,
            incoming,
            connected,
        })
    }

    pub fn send_message(&mut self, message: ClientMessage) {
        if !*self.connected.lock().unwrap() {
            tracing::warn!("Cannot send message, not connected");
            return;
        }

        match bincode::serialize(&message) {
            Ok(data) => {
                tracing::info!(
                    "Queuing message ({} bytes) to server: {:?}",
                    data.len(),
                    message
                );
                if let Err(e) = self.outgoing.send(data) {
                    tracing::error!("Failed to queue message: {}", e);
                    *self.connected.lock().unwrap() = false;
                }
            }
            Err(e) => {
                tracing::error!("Serialize error: {}", e);
            }
        }
    }

    pub fn poll_messages(&mut self) -> Vec<ServerMessage> {
        let mut messages = self.incoming.lock().unwrap();
        messages.drain(..).collect()
    }

    pub fn is_connected(&self) -> bool {
        *self.connected.lock().unwrap()
    }
}

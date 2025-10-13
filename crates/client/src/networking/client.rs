// =============================================================================
// NETWORKING - Client
// =============================================================================

use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;
use std::thread;
use shared::{ClientMessage, ServerMessage};

#[derive(Resource)]
pub struct NetworkClient {
    sender: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
    incoming_messages: Arc<Mutex<VecDeque<ServerMessage>>>,
    connected: bool,
}

impl NetworkClient {
    pub fn connect(server_url: &str) -> Result<Self, String> {
        let (socket, _response) = connect(server_url)
            .map_err(|e| format!("Connection failed: {}", e))?;
        
        let socket = Arc::new(Mutex::new(socket));
        let incoming_messages = Arc::new(Mutex::new(VecDeque::new()));
        
        let socket_clone = socket.clone();
        let messages_clone = incoming_messages.clone();
        
        thread::spawn(move || {
            loop {
                let message = {
                    let mut socket = socket_clone.lock().unwrap();
                    socket.read()
                };
                
                match message {
                    Ok(Message::Binary(data)) => {
                        match bincode::deserialize::<ServerMessage>(&data) {
                            Ok(server_msg) => {
                                messages_clone.lock().unwrap().push_back(server_msg);
                            }
                            Err(e) => {
                                tracing::error!("Deserialize error: {}", e);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        tracing::info!("Server closed connection");
                        break;
                    }
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("Error reading: {}", e);
                        break;
                    }
                }
            }
        });
        
        Ok(Self {
            sender: socket,
            incoming_messages,
            connected: true,
        })
    }
    
    pub fn send_message(&mut self, message: ClientMessage) {
        if !self.connected {
            return;
        }
        
        match bincode::serialize(&message) {
            Ok(data) => {
                let mut socket = self.sender.lock().unwrap();
                if let Err(e) = socket.send(Message::Binary(data)) {
                    tracing::error!("Send error: {}", e);
                    self.connected = false;
                }
            }
            Err(e) => {
                tracing::error!("Serialize error: {}", e);
            }
        }
    }
    
    pub fn poll_messages(&mut self) -> Vec<ServerMessage> {
        let mut messages = self.incoming_messages.lock().unwrap();
        messages.drain(..).collect()
    }
    
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}
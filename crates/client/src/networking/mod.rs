// =============================================================================
// FILE: crates/client/src/networking/mod.rs
// =============================================================================

use bevy::prelude::*;
use std::sync::{Arc, Mutex};
use std::collections::VecDeque;
use tungstenite::{connect, Message, WebSocket};
use tungstenite::stream::MaybeTlsStream;
use std::net::TcpStream;
use std::thread;
use shared::{ClientMessage, ServerMessage};

/// Resource : Client réseau
#[derive(Resource)]
pub struct NetworkClient {
    sender: Arc<Mutex<WebSocket<MaybeTlsStream<TcpStream>>>>,
    incoming_messages: Arc<Mutex<VecDeque<ServerMessage>>>,
    connected: bool,
}

impl NetworkClient {
    /// Crée et connecte le client au serveur
    pub fn connect(server_url: &str) -> Result<Self, String> {
        println!("Connecting to {}...", server_url);
        
        let (socket, response) = connect(server_url)
            .map_err(|e| format!("Connection failed: {}", e))?;
        
        println!("Connected! Response: {:?}", response);
        
        let socket = Arc::new(Mutex::new(socket));
        let incoming_messages = Arc::new(Mutex::new(VecDeque::new()));
        
        // Thread de réception des messages
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
                        // Désérialiser le message
                        match bincode::deserialize::<ServerMessage>(&data) {
                            Ok(server_msg) => {
                                println!("Received message: {:?}", server_msg);
                                messages_clone.lock().unwrap().push_back(server_msg);
                            }
                            Err(e) => {
                                eprintln!("Failed to deserialize message: {}", e);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        println!("Server closed connection");
                        break;
                    }
                    Ok(_) => {} // Ignore autres types
                    Err(e) => {
                        eprintln!("Error reading message: {}", e);
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
    
    /// Envoie un message au serveur
    pub fn send_message(&mut self, message: ClientMessage) {
        if !self.connected {
            eprintln!("Not connected to server!");
            return;
        }
        
        // Sérialiser en binaire
        match bincode::serialize(&message) {
            Ok(data) => {
                let mut socket = self.sender.lock().unwrap();
                if let Err(e) = socket.send(Message::Binary(data)) {
                    eprintln!("Failed to send message: {}", e);
                    self.connected = false;
                }
            }
            Err(e) => {
                eprintln!("Failed to serialize message: {}", e);
            }
        }
    }
    
    /// Récupère les messages reçus
    pub fn poll_messages(&mut self) -> Vec<ServerMessage> {
        let mut messages = self.incoming_messages.lock().unwrap();
        messages.drain(..).collect()
    }
    
    /// Vérifie si connecté
    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

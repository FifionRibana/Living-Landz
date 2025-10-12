// =============================================================================
// FILE: crates/client/src/networking/plugin.rs
// =============================================================================

use bevy::prelude::*;
use super::NetworkClient;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_network_client)
           .add_systems(Update, check_connection_status);
    }
}

fn setup_network_client(mut commands: Commands) {
    println!("Setting up network client...");
    
    // Tentative de connexion au serveur
    match NetworkClient::connect("ws://127.0.0.1:9001") {
        Ok(client) => {
            println!("Successfully connected to server!");
            commands.insert_resource(client);
        }
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            eprintln!("Make sure the server is running on port 9001");
            // Insère un client déconnecté pour éviter les crashes
            // On pourrait aussi quitter l'app ici
        }
    }
}

fn check_connection_status(
    network_client: Option<Res<NetworkClient>>,
    mut last_check: Local<f32>,
    time: Res<Time>,
) {
    *last_check += time.delta_secs();
    
    // Vérifie toutes les 5 secondes
    if *last_check > 5.0 {
        *last_check = 0.0;
        
        if let Some(client) = network_client {
            if !client.is_connected() {
                eprintln!("Lost connection to server!");
            }
        } else {
            eprintln!("No network client available!");
        }
    }
}

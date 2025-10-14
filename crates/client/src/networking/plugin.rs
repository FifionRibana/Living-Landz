// =============================================================================
// NETWORKING - Plugin
// =============================================================================

use bevy::prelude::*;
use super::NetworkClient;

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_network_client);
    }
}

fn setup_network_client(mut commands: Commands) {
    tracing::info!("Attempting connection...");
    match NetworkClient::connect("ws://127.0.0.1:9001") {
        Ok(mut client) => {
            tracing::info!("Connected to server");
            client.send_message(shared::ClientMessage::Login {
                username: "Player".to_string(),
            });
            commands.insert_resource(client);
        }
        Err(e) => {
            tracing:: warn!("Failed to connect: {}", e);
        }
    }
}
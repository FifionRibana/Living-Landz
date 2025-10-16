// =============================================================================
// CRATE SERVER - main.rs
// =============================================================================

use bevy::prelude::*;
use hex_grid::WorldGenerator;
use std::sync::Arc;


mod database;
mod networking;
mod tick;
mod world;

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
    
    let db = database::initialize_database().await;
    let sessions = networking::Sessions::default();
    let world_gen = WorldGenerator::new(12345);
    
    networking::initialize_server(sessions.clone(), world_gen.clone(), Arc::new(db.clone()));

    tokio::task::spawn_blocking(|| {
        App::new()
            .add_plugins(MinimalPlugins)
            .insert_resource(world_gen)
            .insert_resource(sessions)
            .insert_resource(db)
            .run();
    }).await
    .expect("Failed to start Bevy App");

    // Tick system
    // tokio::spawn(async move {
    //     let mut tick: u64 = 0;
    //     loop {
    //         tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    //         tick += 1;
    //         tracing::info!("Tick {}", tick);

    //         let msg = ServerMessage::WorldTick { tick };
    //         broadcast_message(&sessions_clone, msg).await;
    //     }
    // });
}

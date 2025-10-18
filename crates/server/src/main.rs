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

    let (chunk_db, building_db) = database::initialize_database().await;

    let mut map_name = "test_island";
    if args.contains(&"--regenerate-world".to_string())
        || args.contains(&"--generate-world".to_string())
    {
        let result = args
            .iter()
            .find(|arg| arg.starts_with("--map="))
            .map(|arg| {
                map_name = arg.trim_start_matches("--map=");
                tracing::info!("Using map: {}", map_name);
                // Here you would set the map to be used in generation
                map_name
            });
        if result.is_none() {
            tracing::warn!(
                "--map flag provided but no map name found, using default: {}",
                map_name
            );
        }
        map_name = result.unwrap_or(map_name);
    }

    if args.contains(&"--regenerate-world".to_string()) {
        tracing::info!("=== Regenerating World (clearing DB first) ===");

        // Suppression
        chunk_db
            .clear_all_chunks()
            .await
            .expect("Failed to clear DB");

        // Regénération
        world::systems::generate_world_complete(&chunk_db, map_name).await;

        tracing::info!("=== Regeneration Complete ===");
        return;
    }

    if args.contains(&"--generate-world".to_string()) {
        tracing::info!("=== Starting World Generation ===");
        world::systems::generate_world_complete(&chunk_db, map_name).await;
        tracing::info!("=== Generation Complete - Exiting ===");
        return;
    }

    let sessions = networking::Sessions::default();
    let world_gen = WorldGenerator::new(12345);

    networking::initialize_server(
        sessions.clone(),
        world_gen.clone(),
        Arc::new(chunk_db.clone()),
        Arc::new(building_db.clone()),
    );

    tokio::task::spawn_blocking(|| {
        App::new()
            .add_plugins(MinimalPlugins)
            .insert_resource(world_gen)
            .insert_resource(sessions)
            .insert_resource(chunk_db)
            .insert_resource(building_db)
            .run();
    })
    .await
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

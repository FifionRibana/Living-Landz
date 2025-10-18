// =============================================================================
// WORLD SYSTEMS
// =============================================================================

use super::{components::*, generation::*, resources::*};
use crate::database::ChunkDatabase;
use rayon::prelude::*;
use shared::protocol::messages::BuildingData;


pub async fn generate_world_complete(db: &ChunkDatabase, map_name: &str) {
    tracing::info!("Starting world generation...");
    let start = std::time::Instant::now();
    tracing::info!("Using map: {}", map_name);

    // Load maps
    let maps = WorldMaps::load(
        ("assets/maps/".to_string() + map_name + "_heightmap.png").as_str(),
        ("assets/maps/".to_string() + map_name + "_biomemap.png").as_str(),
        ("assets/maps/".to_string() + map_name + "_binarymap.png").as_str(),
        12345,
    )
    .expect("Failed to load world maps");

    let config = maps.config.clone();

    // Generate chunks
    let noise_gen = NoiseGenerator::new(config.seed);
    let total_chunks = (config.chunks_x * config.chunks_y) as usize;

    tracing::info!("Generating {} chunks...", total_chunks);

    let chunks: Vec<(Chunk, Vec<BuildingData>)> = (0..config.chunks_x as i32)
        .flat_map(|x| (0..config.chunks_y as i32).map(move |y| ChunkCoord { x, y }))
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&coord| ChunkGenerator::generate(coord, &maps, &config, &noise_gen))
        .collect();

    tracing::info!(
        "✓ Generated {} chunks in {:?}",
        chunks.len(),
        start.elapsed()
    );

    // Save to DB
    let save_start = std::time::Instant::now();
    for (i, (chunk, buildings)) in chunks.iter().enumerate() {
        db.save_chunk(chunk).await.expect("Failed to save chunk");

        if (i + 1) % 50 == 0 || i + 1 == total_chunks {
            tracing::info!("Progress: {}/{} chunks saved", i + 1, total_chunks);
        }
    }

    tracing::info!("✓ Saved all chunks in {:?}", save_start.elapsed());
    tracing::info!("🎉 Complete! Total: {:?}", start.elapsed());
}

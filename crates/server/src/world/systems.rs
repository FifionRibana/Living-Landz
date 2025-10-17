// =============================================================================
// WORLD SYSTEMS
// =============================================================================

use rayon::prelude::*;
use super::{components::*, resources::*, generation::*};

pub async fn generate_world_complete(
    db: &ChunkDatabase,
) {
    tracing::info!("Starting world generation...");
    let start = std::time::Instant::now();
    
    // Load maps
    let maps = WorldMaps::load(
        "assets/maps/test_island_heightmap.png",
        "assets/maps/test_island_biomes.png",
        "assets/maps/test_island_binary.png",
        12345,
    ).expect("Failed to load world maps");
    
    let config = maps.config.clone();
    
    // Generate chunks
    let noise_gen = NoiseGenerator::new(config.seed);
    let total_chunks = (config.chunks_x * config.chunks_y) as usize;
    
    tracing::info!("Generating {} chunks...", total_chunks);
    
    let chunks: Vec<Chunk> = (0..config.chunks_x as i32)
        .flat_map(|x| {
            (0..config.chunks_y as i32).map(move |y| ChunkCoord { x, y })
        })
        .collect::<Vec<_>>()
        .par_iter()
        .map(|&coord| {
            ChunkGenerator::generate(coord, &maps, &config, &noise_gen)
        })
        .collect();
    
    tracing::info!("✓ Generated {} chunks in {:?}", chunks.len(), start.elapsed());
    
    // Save to DB
    let save_start = std::time::Instant::now();
    for (i, chunk) in chunks.iter().enumerate() {
        db.save_chunk(chunk).await.expect("Failed to save chunk");
        
        if (i + 1) % 50 == 0 || i + 1 == total_chunks {
            tracing::info!("Progress: {}/{} chunks saved", i + 1, total_chunks);
        }
    }
    
    tracing::info!("✓ Saved all chunks in {:?}", save_start.elapsed());
    tracing::info!("🎉 Complete! Total: {:?}", start.elapsed());
}
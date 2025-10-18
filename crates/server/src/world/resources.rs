// =============================================================================
// WORLD RESOURCES
// =============================================================================

use bevy::prelude::*;
use image::{DynamicImage, GenericImageView, Rgba};
use shared::types::*;

pub const CHUNK_SIZE: u32 = 60;

// ============================================================================
// WORLD CONFIGURATION
// ============================================================================

#[derive(Resource, Clone)]
pub struct WorldConfig {
    pub map_width: u32,
    pub map_height: u32,
    pub chunks_x: u32,
    pub chunks_y: u32,
    pub seed: u32,
}

// ============================================================================
// WORLD MAPS
// ============================================================================

#[derive(Resource)]
pub struct WorldMaps {
    pub heightmap: DynamicImage,
    pub biome_map: DynamicImage,
    pub binary_map: DynamicImage,
    pub config: WorldConfig,
}

impl WorldMaps {
    pub fn load(
        heightmap_path: &str,
        biome_path: &str,
        binary_path: &str,
        seed: u32,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        tracing::info!("Loading world maps...");

        let heightmap = image::open(heightmap_path)?;
        let biome_map = image::open(biome_path)?;
        let binary_map = image::open(binary_path)?;

        let width = heightmap.width();
        let height = heightmap.height();

        if biome_map.width() != width || biome_map.height() != height {
            return Err("All maps must have same dimensions".into());
        }

        let config = WorldConfig {
            map_width: width,
            map_height: height,
            chunks_x: width / CHUNK_SIZE,
            chunks_y: height / CHUNK_SIZE,
            seed,
        };

        tracing::info!(
            "✓ Maps: {}x{} → {}x{} chunks",
            width,
            height,
            config.chunks_x,
            config.chunks_y
        );

        Ok(Self {
            heightmap,
            biome_map,
            binary_map,
            config,
        })
    }

    pub fn sample_height(&self, pixel_x: u32, pixel_y: u32) -> f32 {
        let pixel = self.heightmap.get_pixel(pixel_x, pixel_y);
        pixel[0] as f32 / 255.0
    }

    pub fn is_land(&self, pixel_x: u32, pixel_y: u32) -> bool {
        let pixel = self.binary_map.get_pixel(pixel_x, pixel_y);
        pixel[0] >= 225
    }

    pub fn is_lake(&self, pixel_x: u32, pixel_y: u32) -> bool {
        let pixel = self.binary_map.get_pixel(pixel_x, pixel_y);
        pixel[0] >= 25 && pixel[0] < 225
    }

    pub fn is_ocean(&self, pixel_x: u32, pixel_y: u32) -> bool {
        let pixel = self.binary_map.get_pixel(pixel_x, pixel_y);
        pixel[0] < 25
    }

    pub fn sample_biome(&self, pixel_x: u32, pixel_y: u32) -> BiomeType {
        let Rgba([r, g, b, _]) = self.biome_map.get_pixel(pixel_x, pixel_y);

        // TODO: Adapter selon ta palette
        // Hot desert: rgb(251, 231, 159)
        // Savanna: rgb(210, 208, 130)
        // Grassland: rgb(200, 214, 143)
        // Tropical seasonal forest: rgb(182, 217, 93)
        // Tropical rainforest: rgb(125, 203, 52)
        // Tropical deciduous forest: rgb(41, 188, 86)
        // Temperate rainforest: rgb(64, 156, 67)
        // Wetland: rgb(11, 145, 49)
        // Taiga: rgb(75, 107, 50)
        // Tundra: rgb(150, 120, 75)
        // Cold desert: rgb(181, 184, 135)
        // Glacier: rgb(213, 231, 235)
        //
        match (r, g, b) {
            (0, 0, 0) => BiomeType::Ocean,
            // (200..=255, 200..=255, 150..=200) => BiomeType::Beach,
            (210, 208, 130) => BiomeType::Savanna,
            (200, 214, 143) => BiomeType::Grassland,
            (182, 217, 93) => BiomeType::Forest,
            (125, 203, 52) => BiomeType::Forest,
            (41, 188, 86) => BiomeType::Forest,
            (64, 156, 67) => BiomeType::Forest,
            (11, 145, 49) => BiomeType::Swamp,
            (75, 107, 50) => BiomeType::Taiga,
            // (0..=50, 80..=120, 0..=30) => BiomeType::DenseForest,
            // (150..=200, 150..=200, 150..=200) => BiomeType::Mountain,
            // (200..=255, 200..=255, 200..=255) => BiomeType::HighMountain,
            (251, 231, 159) => BiomeType::Desert,
            (181, 184, 135) => BiomeType::ColdDesert,
            (150, 120, 75) => BiomeType::Tundra,
            (213, 231, 235) => BiomeType::Ice,
            _ => BiomeType::Grassland,
        }
        // match (r, g, b) {
        //     (0..=30, 0..=50, 100..=255) => BiomeType::DeepOcean,
        //     (0..=50, 50..=100, 150..=255) => BiomeType::Ocean,
        //     (200..=255, 200..=255, 150..=200) => BiomeType::Beach,
        //     (50..=150, 100..=200, 50..=100) => BiomeType::Grassland,
        //     (0..=80, 100..=150, 0..=50) => BiomeType::Forest,
        //     (0..=50, 80..=120, 0..=30) => BiomeType::DenseForest,
        //     (150..=200, 150..=200, 150..=200) => BiomeType::Mountain,
        //     (200..=255, 200..=255, 200..=255) => BiomeType::HighMountain,
        //     (220..=255, 200..=230, 100..=150) => BiomeType::Desert,
        //     (180..=220, 220..=255, 220..=255) => BiomeType::Tundra,
        //     _ => BiomeType::Grassland,
        // }
    }
}

// ============================================================================
// DATABASE
// ============================================================================


// ============================================================================
// GENERATION PROGRESS
// ============================================================================

#[derive(Resource, Default)]
pub struct GenerationProgress {
    pub total_chunks: usize,
    pub generated: usize,
    pub saved: usize,
}

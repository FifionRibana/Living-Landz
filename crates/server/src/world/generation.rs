// =============================================================================
// WORLD GENERATION
// =============================================================================

use noise::{NoiseFn, Perlin};
use shared::types::*;
use super::{components::*, resources::*};

// ============================================================================
// CHUNK GENERATOR
// ============================================================================

pub struct ChunkGenerator;

impl ChunkGenerator {
    pub fn generate(
        chunk_coord: ChunkCoord,
        maps: &WorldMaps,
        config: &WorldConfig,
        noise_gen: &NoiseGenerator,
    ) -> Chunk {
        let mut tiles = Vec::with_capacity((CHUNK_SIZE * CHUNK_SIZE) as usize);
        
        for local_q in 0..CHUNK_SIZE as i32 {
            for local_r in 0..CHUNK_SIZE as i32 {
                let world_q = chunk_coord.x * CHUNK_SIZE as i32 + local_q;
                let world_r = chunk_coord.y * CHUNK_SIZE as i32 + local_r;
                let hex = HexCoord { q: world_q, r: world_r };
                
                let tile = Self::generate_tile(hex, maps, config, noise_gen);
                tiles.push(tile);
            }
        }
        
        Chunk {
            coord: chunk_coord,
            tiles,
            generated_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    fn generate_tile(
        hex: HexCoord,
        maps: &WorldMaps,
        config: &WorldConfig,
        noise: &NoiseGenerator,
    ) -> HexTile {
        let (pixel_x, pixel_y) = Self::hex_to_pixel(hex, config);
        
        // Binary map check
        if !maps.is_land(pixel_x, pixel_y) {
            return HexTile {
                coord: hex,
                biome: BiomeType::DeepOcean,
                altitude: -500,
                quality: 0,
                is_river: false,
                river_flow: None,
            };
        }
        
        // Génération terrain
        let height_value = maps.sample_height(pixel_x, pixel_y);
        let base_biome = maps.sample_biome(pixel_x, pixel_y);
        let local_noise = noise.sample_terrain(hex);
        let smoothed_height = height_value + local_noise * 0.15;
        let altitude = Self::height_to_altitude(smoothed_height);
        let biome = Self::refine_biome(base_biome, altitude, smoothed_height);
        let quality = Self::calculate_quality(hex, biome, noise);
        
        HexTile {
            coord: hex,
            biome,
            altitude,
            quality,
            is_river: false,
            river_flow: None,
        }
    }
    
    fn hex_to_pixel(hex: HexCoord, config: &WorldConfig) -> (u32, u32) {
        let scale_x = config.map_width as f32 / (config.chunks_x as f32 * CHUNK_SIZE as f32);
        let scale_y = config.map_height as f32 / (config.chunks_y as f32 * CHUNK_SIZE as f32);
        
        let x = (hex.q as f32 * scale_x).clamp(0.0, config.map_width as f32 - 1.0) as u32;
        let y = (hex.r as f32 * scale_y).clamp(0.0, config.map_height as f32 - 1.0) as u32;
        
        (x, y)
    }
    
    fn height_to_altitude(height: f32) -> i16 {
        if height < 0.4 {
            ((height / 0.4) * -990.0 - 1000.0) as i16
        } else if height < 0.5 {
            (((height - 0.4) / 0.1) * 15.0 - 10.0) as i16
        } else if height < 0.8 {
            (((height - 0.5) / 0.3) * 495.0 + 5.0) as i16
        } else {
            (((height - 0.8) / 0.2) * 2500.0 + 500.0) as i16
        }
    }
    
    fn refine_biome(base: BiomeType, altitude: i16, height: f32) -> BiomeType {
        if altitude < -500 {
            BiomeType::DeepOcean
        } else if altitude < -10 {
            BiomeType::Ocean
        } else if altitude < 5 && height < 0.52 {
            BiomeType::Beach
        } else if altitude < 10 {
            BiomeType::Coast
        } else if altitude > 1500 {
            BiomeType::HighMountain
        } else if altitude > 800 {
            BiomeType::Mountain
        } else {
            base
        }
    }
    
    fn calculate_quality(hex: HexCoord, biome: BiomeType, noise: &NoiseGenerator) -> u8 {
        let base_quality = match biome {
            BiomeType::Grassland => 75,
            BiomeType::Forest => 65,
            BiomeType::Coast => 70,
            BiomeType::Beach => 40,
            BiomeType::Mountain => 30,
            BiomeType::Desert => 20,
            BiomeType::Tundra => 25,
            BiomeType::Swamp => 50,
            _ => 10,
        };
        
        let variation = noise.sample_quality(hex) * 30.0;
        (base_quality as f32 + variation).clamp(0.0, 100.0) as u8
    }
}

// ============================================================================
// NOISE GENERATOR
// ============================================================================

pub struct NoiseGenerator {
    terrain_noise: Perlin,
    quality_noise: Perlin,
}

impl NoiseGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            terrain_noise: Perlin::new(seed),
            quality_noise: Perlin::new(seed + 1),
        }
    }
    
    pub fn sample_terrain(&self, hex: HexCoord) -> f32 {
        let scale = 0.05;
        self.terrain_noise.get([hex.q as f64 * scale, hex.r as f64 * scale]) as f32
    }
    
    pub fn sample_quality(&self, hex: HexCoord) -> f32 {
        let scale = 0.02;
        self.quality_noise.get([hex.q as f64 * scale, hex.r as f64 * scale]) as f32
    }
}
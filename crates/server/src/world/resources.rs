// =============================================================================
// WORLD RESOURCES
// =============================================================================

use super::components::*;
use bevy::prelude::*;
use image::{DynamicImage, GenericImageView, Rgba};
use shared::types::*;
use sqlx::{PgPool, Row};

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
        pixel[0] > 127
    }

    pub fn sample_biome(&self, pixel_x: u32, pixel_y: u32) -> BiomeType {
        let Rgba([r, g, b, _]) = self.biome_map.get_pixel(pixel_x, pixel_y);

        // TODO: Adapter selon ta palette
        match (r, g, b) {
            (0..=30, 0..=50, 100..=255) => BiomeType::DeepOcean,
            (0..=50, 50..=100, 150..=255) => BiomeType::Ocean,
            (200..=255, 200..=255, 150..=200) => BiomeType::Beach,
            (50..=150, 100..=200, 50..=100) => BiomeType::Grassland,
            (0..=80, 100..=150, 0..=50) => BiomeType::Forest,
            (0..=50, 80..=120, 0..=30) => BiomeType::DenseForest,
            (150..=200, 150..=200, 150..=200) => BiomeType::Mountain,
            (200..=255, 200..=255, 200..=255) => BiomeType::HighMountain,
            (220..=255, 200..=230, 100..=150) => BiomeType::Desert,
            (180..=220, 220..=255, 220..=255) => BiomeType::Tundra,
            _ => BiomeType::Grassland,
        }
    }
}

// ============================================================================
// DATABASE
// ============================================================================

#[derive(Resource, Clone)]
pub struct ChunkDatabase {
    pool: PgPool,
}

impl ChunkDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn init_schema(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS chunks (
                chunk_x INT NOT NULL,
                chunk_y INT NOT NULL,
                tiles BYTEA NOT NULL,
                generated_at BIGINT NOT NULL,
                PRIMARY KEY (chunk_x, chunk_y)
            )
            "#
        )
        .execute(&self.pool)
        .await?;
        
        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_chunks_generated ON chunks(generated_at)"
        )
        .execute(&self.pool)
        .await?;

        tracing::info!("✓ Database schema ready");
        Ok(())
    }

    pub async fn save_chunk(&self, chunk: &Chunk) -> Result<(), sqlx::Error> {
        let tiles_bytes = bincode::serialize(&chunk.tiles).expect("Failed to serialize tiles");

        sqlx::query(
            r#"
            INSERT INTO chunks (chunk_x, chunk_y, tiles, generated_at)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (chunk_x, chunk_y) 
            DO UPDATE SET tiles = $3, generated_at = $4
            "#,
        )
        .bind(chunk.coord.x)
        .bind(chunk.coord.y)
        .bind(&tiles_bytes)
        .bind(chunk.generated_at as i64)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn load_chunk(&self, coord: ChunkCoord) -> Result<Option<Chunk>, sqlx::Error> {
        let row = sqlx::query(
            "SELECT tiles, generated_at FROM chunks WHERE chunk_x = $1 AND chunk_y = $2",
        )
        .bind(coord.x)
        .bind(coord.y)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let tiles_bytes: Vec<u8> = r.get("tiles");
            let tiles: Vec<HexTile> =
                bincode::deserialize(&tiles_bytes).expect("Failed to deserialize tiles");

            Chunk {
                coord,
                tiles,
                generated_at: r.get::<i64, _>("generated_at") as u64,
            }
        }))
    }

    pub async fn chunk_exists(&self, coord: ChunkCoord) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("SELECT 1 FROM chunks WHERE chunk_x = $1 AND chunk_y = $2")
            .bind(coord.x)
            .bind(coord.y)
            .fetch_optional(&self.pool)
            .await?;

        Ok(result.is_some())
    }
}

// ============================================================================
// GENERATION PROGRESS
// ============================================================================

#[derive(Resource, Default)]
pub struct GenerationProgress {
    pub total_chunks: usize,
    pub generated: usize,
    pub saved: usize,
}

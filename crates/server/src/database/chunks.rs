use bevy::prelude::*;

use sqlx::PgPool;
use sqlx::Row;

use crate::world::{Chunk, ChunkCoord, HexTile};

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
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query("CREATE INDEX IF NOT EXISTS idx_chunks_generated ON chunks(generated_at)")
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

    pub async fn clear_all_chunks(&self) -> Result<(), sqlx::Error> {
        tracing::warn!("🗑️  Clearing all chunks from database...");

        sqlx::query("DELETE FROM chunks")
            .execute(&self.pool)
            .await?;

        tracing::info!("✓ Database cleared");
        Ok(())
    }

    /// Compte le nombre de chunks en DB
    pub async fn count_chunks(&self) -> Result<i64, sqlx::Error> {
        let row = sqlx::query("SELECT COUNT(*) as count FROM chunks")
            .fetch_one(&self.pool)
            .await?;

        Ok(row.get("count"))
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

use shared::types::*;
use shared::protocol::messages::BuildingData;

use bevy::prelude::*;
use sqlx::PgPool;
use sqlx::Row;

use crate::world::ChunkCoord;

#[derive(Resource, Clone)]
pub struct BuildingDatabase {
    pool: PgPool,
}

impl BuildingDatabase {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn init_schema(&self) -> Result<(), sqlx::Error> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS buildings (
                id BIGINT PRIMARY KEY,
                chunk_x INT NOT NULL,
                chunk_y INT NOT NULL,
                coord_q INT NOT NULL,
                coord_r INT NOT NULL,
                
                category VARCHAR(32) NOT NULL,
                variant VARCHAR(64) NOT NULL,
                
                health REAL NOT NULL,
                max_health REAL NOT NULL,
                construction_progress REAL DEFAULT 1.0,
                owner_id BIGINT,
                
                metadata JSONB,
                
                created_at BIGINT NOT NULL,
                last_modified BIGINT NOT NULL
            )
            "#,
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_buildings_chunk ON buildings(chunk_x, chunk_y)",
        )
        .execute(&self.pool)
        .await?;

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS idx_buildings_coord ON buildings(coord_q, coord_r)",
        )
        .execute(&self.pool)
        .await?;

        tracing::info!("✓ Buildings database schema ready");
        Ok(())
    }

    pub async fn save_buildings(
        &self,
        chunk_coord: ChunkCoord,
        buildings: &[BuildingData],
    ) -> Result<(), sqlx::Error> {
        for building_data in buildings {
            let building = &building_data.building;
            let metadata_bytes =
                bincode::serialize(building_data).expect("Failed to serialize building data");

            sqlx::query(
                r#"
                INSERT INTO buildings (
                    id, chunk_x, chunk_y, coord_q, coord_r,
                    category, variant, health, max_health,
                    construction_progress, owner_id, metadata,
                    created_at, last_modified
                ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                ON CONFLICT (id) DO UPDATE SET
                    health = $8,
                    last_modified = $14,
                    metadata = $12
                "#,
            )
            .bind(building.id as i64)
            .bind(chunk_coord.x)
            .bind(chunk_coord.y)
            .bind(building.coord.q)
            .bind(building.coord.r)
            .bind(format!("{:?}", building.building_type.category))
            .bind(&building.building_type.variant)
            .bind(building.health)
            .bind(building.max_health)
            .bind(building.construction_progress)
            .bind(building.owner_id.map(|id| id as i64))
            .bind(&metadata_bytes)
            .bind(building.created_at as i64)
            .bind(building.last_modified as i64)
            .execute(&self.pool)
            .await?;
        }

        Ok(())
    }

    pub async fn load_buildings(
        &self,
        chunk_coord: ChunkCoord,
    ) -> Result<Vec<BuildingData>, sqlx::Error> {
        let rows =
            sqlx::query("SELECT metadata FROM buildings WHERE chunk_x = $1 AND chunk_y = $2")
                .bind(chunk_coord.x)
                .bind(chunk_coord.y)
                .fetch_all(&self.pool)
                .await?;

        Ok(rows
            .iter()
            .filter_map(|r| {
                let bytes: Vec<u8> = r.get("metadata");
                bincode::deserialize(&bytes).ok()
            })
            .collect())
    }
}

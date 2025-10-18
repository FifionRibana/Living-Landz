use super::generation::NoiseGenerator;
use shared::protocol::BuildingData;
use shared::types::*;

pub struct BuildingGenerator;

impl BuildingGenerator {
    /// Vérifie si un biome peut spawner des arbres
    pub fn can_spawn_trees(biome: BiomeType) -> bool {
        matches!(
            biome,
            BiomeType::Forest
                | BiomeType::DenseForest
                | BiomeType::Taiga
                | BiomeType::Grassland
                | BiomeType::Swamp
                | BiomeType::Savanna
                | BiomeType::Mountain
        )
    }

    /// Génère un arbre pour une tuile donnée
    pub fn generate_tree(
        coord: HexCoord,
        biome: BiomeType,
        altitude: i16,
        noise: &NoiseGenerator,
    ) -> Option<BuildingData> {
        // Probabilité basée sur biome
        let spawn_chance = match biome {
            BiomeType::DenseForest => 0.85,
            BiomeType::Forest => 0.60,
            BiomeType::Taiga => 0.40,
            BiomeType::Grassland => 0.30,
            BiomeType::Swamp => 0.45,
            BiomeType::Savanna => 0.1,
            BiomeType::Mountain => 0.40,
            _ => return None,
        };

        let noise_val = noise.sample_terrain(coord);
        if noise_val < (spawn_chance - 1.0) {
            // tracing::debug!("No tree spawn on {}, {}", coord.q, coord.r);
            return None;
        }

        // ID déterministe basé sur coordonnées
        let id = Self::generate_building_id(coord);

        // Âge basé sur altitude + noise
        let age_noise = noise.sample_quality(coord);
        let age = match (altitude, age_noise) {
            (a, n) if a > 500.0 as i16 && n < -0.3 => TreeAge::Secular,
            (a, n) if a > 300.0 as i16 && n < 0.0 => TreeAge::Ancient,
            (_, n) if n < 0.2 => TreeAge::Adult,
            (_, n) if n < 0.5 => TreeAge::Mature,
            (_, n) if n < 0.7 => TreeAge::Young,
            _ => TreeAge::Sapling,
        };

        // Type de bois selon biome/altitude
        let wood_type = WoodType::from_biome(biome, altitude);

        // Densité (sera calculée après génération complète du chunk)
        let density = 0.5; // Placeholder, calculé dans calculate_forest_density

        let tree_data = TreeData {
            age,
            density,
            wood_type,
            yield_multiplier: density * age.health_multiplier(),
        };

        let health = 100.0 * age.health_multiplier();

        let building = Building {
            id,
            building_type: BuildingType::tree(wood_type),
            coord,
            health,
            max_health: health,
            construction_progress: 1.0,
            owner_id: None,
            created_at: Self::timestamp(),
            last_modified: Self::timestamp(),
        };
        // tracing::debug!("Spawning tree on {}, {} (biome: {:?}, altitude: {}, age: {:?}, density: {})", coord.q, coord.r, biome, altitude, age, density);

        Some(BuildingData {
            building,
            tree_data: Some(tree_data),
        })
    }

    /// Calcule la densité d'un arbre basée sur ses voisins
    pub fn calculate_forest_density(
        coord: HexCoord,
        all_trees: &std::collections::HashMap<HexCoord, &BuildingData>,
    ) -> f32 {
        let neighbors = coord.neighbors();
        let forest_neighbors = neighbors
            .iter()
            .filter(|n| all_trees.contains_key(n))
            .count();

        // Plus de voisins forêt = plus dense
        ((forest_neighbors as f32 / 6.0) * 0.7 + 0.3).clamp(0.3, 1.0)
    }

    /// Génère un ID déterministe pour un bâtiment
    fn generate_building_id(coord: HexCoord) -> u64 {
        let mut hash = 0x517cc1b727220a95u64;
        hash ^= coord.q as u64;
        hash = hash.wrapping_mul(0x3243f6a8885a308d);
        hash ^= coord.r as u64;
        hash = hash.wrapping_mul(0x3243f6a8885a308d);
        hash
    }

    fn timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

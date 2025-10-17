use serde::{Deserialize, Serialize};
use super::coords::HexCoord;

/// Catégorie de bâtiment (extensible)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BuildingCategory {
    Natural,        // Arbres, rochers
    Structure,      // Maisons, fermes
    Infrastructure, // Routes, ponts
    Defense,        // Murailles, tours
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildingActionType {
    StartDestruction,
    CancelDestruction,
    Repair,
    Upgrade,
}

/// Type de bâtiment
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BuildingType {
    pub category: BuildingCategory,
    pub variant: String,  // "oak_tree", "pine_tree", "stone_house", etc.
}

impl BuildingType {
    pub fn tree(wood_type: WoodType) -> Self {
        Self {
            category: BuildingCategory::Natural,
            variant: format!("{:?}_tree", wood_type).to_lowercase(),
        }
    }
}

/// État d'un bâtiment (partagé client-serveur)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Building {
    pub id: u64,
    pub building_type: BuildingType,
    pub coord: HexCoord,
    
    // Attributs génériques
    pub health: f32,
    pub max_health: f32,
    pub construction_progress: f32, // 0.0-1.0 (1.0 = construit)
    pub owner_id: Option<u64>,
    
    // Métadonnées
    pub created_at: u64,
    pub last_modified: u64,
}

/// Données spécifiques aux arbres
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeData {
    pub age: TreeAge,
    pub density: f32,           // 0.0-1.0 (affect rendu + ressources)
    pub wood_type: WoodType,
    pub yield_multiplier: f32,  // Basé sur densité + âge
}

/// Âge d'un arbre (影響 visuel et ressources)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TreeAge {
    Sapling,    // 0-5 ans - petit
    Young,      // 5-20 ans - moyen
    Mature,     // 20-50 ans - grand
    Adult,      // 50-100 ans - très grand
    Ancient,    // 100-200 ans - énorme
    Secular,    // 200+ ans - géant légendaire
}

impl TreeAge {
    /// Multiplicateur de santé basé sur l'âge
    pub fn health_multiplier(&self) -> f32 {
        match self {
            TreeAge::Sapling => 0.2,
            TreeAge::Young => 0.5,
            TreeAge::Mature => 0.8,
            TreeAge::Adult => 1.0,
            TreeAge::Ancient => 1.5,
            TreeAge::Secular => 2.0,
        }
    }
    
    /// Multiplicateur de scale visuel
    pub fn scale_multiplier(&self) -> f32 {
        match self {
            TreeAge::Sapling => 0.3,
            TreeAge::Young => 0.6,
            TreeAge::Mature => 0.9,
            TreeAge::Adult => 1.0,
            TreeAge::Ancient => 1.2,
            TreeAge::Secular => 1.4,
        }
    }
}

/// Type de bois
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WoodType {
    Oak,
    Pine,
    Birch,
    Maple,
    Spruce,
    Cedar,
    // Extensible pour 12+ essences
}

impl WoodType {
    /// Type de bois selon le biome
    pub fn from_biome(biome: super::biomes::BiomeType, altitude: i16) -> Self {
        use super::biomes::BiomeType;
        match biome {
            BiomeType::Taiga => WoodType::Spruce,
            BiomeType::Forest if altitude > 400 => WoodType::Birch,
            BiomeType::Forest => WoodType::Oak,
            BiomeType::DenseForest if altitude > 300 => WoodType::Pine,
            BiomeType::DenseForest => WoodType::Oak,
            _ => WoodType::Oak,
        }
    }
}
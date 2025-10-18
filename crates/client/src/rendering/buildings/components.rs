use bevy::prelude::*;
use shared::types::*;

/// Marqueur pour un bâtiment
#[derive(Component)]
pub struct BuildingEntity {
    pub id: u64,
    pub coord: HexCoord,
    pub building_type: BuildingType,
}

/// Données visuelles d'un bâtiment
#[derive(Component)]
pub struct BuildingVisuals {
    pub sprite_index: usize,     // Index dans atlas (futur)
    pub variation: u8,            // 0-7 pour 8 variations
    pub scale: f32,               // Basé sur âge
    pub z_offset: f32,            // Sorting visuel
}

/// Densité visuelle pour arbres
#[derive(Component)]
pub struct VisualDensity {
    pub base_sprites: u8,         // Nombre de sprites à afficher
    pub scatter_pattern: u64,     // Seed pour répartition
}

/// Animation en cours (futur)
#[derive(Component)]
pub enum BuildingAnimation {
    None,
    Constructing { progress: f32 },
    Destroying { progress: f32 },
    Idle { frame: u8 },
}
use bevy::prelude::*;
use std::collections::HashMap;
use shared::types::*;

/// Resource pour les matériaux de bâtiments (placeholder couleurs)
#[derive(Resource)]
pub struct BuildingMaterials {
    pub materials: HashMap<WoodType, Handle<ColorMaterial>>,
}

impl BuildingMaterials {
    pub fn create(mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        let wood_colors = [
            (WoodType::Oak, Color::srgb(0.55, 0.35, 0.2)),      // Chêne brun
            (WoodType::Pine, Color::srgb(0.7, 0.5, 0.3)),       // Pin clair
            (WoodType::Birch, Color::srgb(0.9, 0.85, 0.7)),     // Bouleau blanc
            (WoodType::Maple, Color::srgb(0.6, 0.4, 0.25)),     // Érable
            (WoodType::Spruce, Color::srgb(0.45, 0.3, 0.2)),    // Épicéa foncé
            (WoodType::Cedar, Color::srgb(0.65, 0.45, 0.3)),    // Cèdre rougeâtre
        ];
        
        let mut material_map = HashMap::new();
        
        for (wood_type, color) in wood_colors {
            let material = materials.add(ColorMaterial::from(color));
            material_map.insert(wood_type, material);
        }
        
        Self {
            materials: material_map,
        }
    }
    
    pub fn get_material(&self, wood_type: WoodType) -> Handle<ColorMaterial> {
        self.materials.get(&wood_type)
            .cloned()
            .unwrap_or_else(|| self.materials[&WoodType::Oak].clone())
    }
}
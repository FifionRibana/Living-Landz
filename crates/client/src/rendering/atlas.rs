use bevy::prelude::*;
use std::collections::HashMap;
use shared::BiomeType;
use super::hex_mesh::create_hexagon_mesh;
use super::hex_config::HexConfig;

#[derive(Resource)]
pub struct BiomeMaterials {
    pub materials: HashMap<BiomeType, Handle<ColorMaterial>>,
    pub hex_mesh: Handle<Mesh>,
}

impl BiomeMaterials {
    pub fn create(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        hex_config: Res<HexConfig>,
    ) -> Self {
        // Créer le mesh hexagonal avec hexx
        let hex_mesh = meshes.add(create_hexagon_mesh(hex_config.layout.clone(), hex_config.hex_radius));
        
        // Couleurs par biome
        // let biome_colors = [
        //     (BiomeType::DeepOcean, Color::srgb(0.0, 0.3, 0.8)),
        //     (BiomeType::Ocean, Color::srgb(0.0, 0.4, 0.8)),
        //     (BiomeType::Coast, Color::srgb(0.5, 0.7, 0.9)),
        //     (BiomeType::Beach, Color::srgb(0.9, 0.85, 0.7)),
        //     (BiomeType::Lake, Color::srgb(0.2, 0.5, 0.9)),      // ← NOUVEAU
        //     (BiomeType::Grassland, Color::srgb(0.2, 0.8, 0.2)),
        //     (BiomeType::Forest, Color::srgb(0.0, 0.5, 0.0)),
        //     (BiomeType::DenseForest, Color::srgb(0.0, 0.3, 0.0)),
        //     (BiomeType::Mountain, Color::srgb(0.5, 0.5, 0.5)),
        //     (BiomeType::HighMountain, Color::srgb(0.8, 0.8, 0.8)),
        //     (BiomeType::Desert, Color::srgb(0.9, 0.8, 0.5)),
        //     (BiomeType::Tundra, Color::srgb(0.8, 0.9, 0.9)),
        //     (BiomeType::Taiga, Color::srgb(0.3, 0.5, 0.4)),
        //     (BiomeType::Swamp, Color::srgb(0.4, 0.5, 0.3)),
        //     (BiomeType::Ice, Color::srgb(1.0, 1.0, 1.0)),
        // ];

        let biome_colors = [
            (BiomeType::Ocean, Color::srgb_u8(0, 80, 230)),
            (BiomeType::DeepOcean, Color::srgb_u8(0, 25, 135)),
            (BiomeType::Coast, Color::srgb_u8(130, 180, 240)),
            (BiomeType::Beach, Color::srgb_u8(238, 225, 198)),
            (BiomeType::Lake, Color::srgb_u8(55, 138, 236)),
            (BiomeType::Desert, Color::srgb_u8(251, 231, 159)),
            (BiomeType::Savanna, Color::srgb_u8(210, 208, 130)),
            (BiomeType::Grassland, Color::srgb_u8(200, 214, 143)),
            (BiomeType::Forest, Color::srgb_u8(41, 188, 86)),
            (BiomeType::DenseForest, Color::srgb_u8(64, 156, 67)),
            (BiomeType::Swamp, Color::srgb_u8(11, 145, 49)),
            (BiomeType::Taiga, Color::srgb_u8(75, 107, 50)),
            (BiomeType::Tundra, Color::srgb_u8(150, 120, 75)),
            (BiomeType::ColdDesert, Color::srgb_u8(181, 184, 135)),
            (BiomeType::Ice, Color::srgb_u8(213, 231, 235)),
            (BiomeType::Mountain, Color::srgb(0.5, 0.5, 0.5)),
            (BiomeType::HighMountain, Color::srgb(0.8, 0.8, 0.8)),
        ];
        
        let mut material_map = HashMap::new();
        
        for (biome, color) in biome_colors {
            let material = materials.add(ColorMaterial::from(color));
            material_map.insert(biome, material);
        }
        
        Self {
            materials: material_map,
            hex_mesh,
        }
    }
    
    pub fn get_material(&self, biome: BiomeType) -> Handle<ColorMaterial> {
        self.materials.get(&biome)
            .cloned()
            .unwrap_or_else(|| self.materials[&BiomeType::Grassland].clone())
    }
}
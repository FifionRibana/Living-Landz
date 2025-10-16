use bevy::prelude::*;
use shared::BiomeType;
use std::collections::HashMap;

#[derive(Resource)]
pub struct TerrainAtlas {
    pub texture: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,

    /// Mapping biome → index dans l'atlas
    pub biome_indices: HashMap<BiomeType, usize>,
}

impl TerrainAtlas {
    /// Crée un atlas temporaire avec des couleurs pour chaque biome
    /// TODO: Remplacer par vraies textures
    pub fn create_placeholder(
        mut images: ResMut<Assets<Image>>,
        mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        use bevy::asset::RenderAssetUsages;
        use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

        // Créer une image 768x48 (16 biomes × 48px)
        const TILE_SIZE: u32 = 48;
        const BIOME_COUNT: u32 = 14;

        let mut image_data = vec![255u8; (TILE_SIZE * BIOME_COUNT * TILE_SIZE * 4) as usize];

        // Couleurs par biome (RGBA)
        let biome_colors = [
            ([0, 76, 204, 255], BiomeType::DeepOcean),
            ([0, 102, 204, 255], BiomeType::Ocean),
            ([128, 179, 230, 255], BiomeType::Coast),
            ([230, 217, 179, 255], BiomeType::Beach),
            ([51, 204, 51, 255], BiomeType::Grassland),
            ([0, 128, 0, 255], BiomeType::Forest),
            ([0, 77, 0, 255], BiomeType::DenseForest),
            ([128, 128, 128, 255], BiomeType::Mountain),
            ([204, 204, 204, 255], BiomeType::HighMountain),
            ([230, 204, 128, 255], BiomeType::Desert),
            ([204, 230, 230, 255], BiomeType::Tundra),
            ([77, 128, 102, 255], BiomeType::Taiga),
            ([102, 128, 77, 255], BiomeType::Swamp),
            ([255, 255, 255, 255], BiomeType::Ice),
        ];

        // Remplir chaque tile avec sa couleur
        for (idx, (color, _)) in biome_colors.iter().enumerate() {
            let tile_offset_x = (idx as u32 * TILE_SIZE) as usize;

            for y in 0..TILE_SIZE {
                for x in 0..TILE_SIZE {
                    let pixel_x = tile_offset_x + x as usize;
                    let pixel_idx = ((y * TILE_SIZE * BIOME_COUNT + pixel_x as u32) * 4) as usize;

                    image_data[pixel_idx] = color[0];
                    image_data[pixel_idx + 1] = color[1];
                    image_data[pixel_idx + 2] = color[2];
                    image_data[pixel_idx + 3] = color[3];
                }
            }
        }

        let image = Image::new(
            Extent3d {
                width: TILE_SIZE * BIOME_COUNT,
                height: TILE_SIZE,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            image_data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );

        let texture_handle = images.add(image);

        // Créer layout
        let layout =
            TextureAtlasLayout::from_grid(UVec2::splat(TILE_SIZE), BIOME_COUNT, 1, None, None);
        let layout_handle = layouts.add(layout);

        // Mapper biomes → indices
        let mut biome_indices = HashMap::new();
        for (idx, (_, biome)) in biome_colors.iter().enumerate() {
            biome_indices.insert(*biome, idx);
        }

        Self {
            texture: texture_handle,
            layout: layout_handle,
            biome_indices,
        }
    }

    pub fn get_index(&self, biome: BiomeType) -> usize {
        *self.biome_indices.get(&biome).unwrap_or(&0)
    }
}

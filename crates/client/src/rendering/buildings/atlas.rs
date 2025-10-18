use bevy::prelude::*;
use std::collections::HashMap;
use shared::types::buildings::WoodType;

#[derive(Resource)]
pub struct TreeAtlas {
    /// Sprites par essence: [variation_1, variation_2, ...]
    pub sprites: HashMap<WoodType, Vec<Handle<Image>>>,
}

impl TreeAtlas {
    pub fn new() -> Self {
        Self {
            sprites: HashMap::new(),
        }
    }

    pub fn get_variations(&self, wood_type: WoodType) -> Option<&[Handle<Image>]> {
        self.sprites.get(&wood_type).map(|v| v.as_slice())
    }
}

pub fn setup_tree_atlas(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let mut atlas = TreeAtlas::new();

    let wood_types = [
        (WoodType::Oak, "oak_tree", 6), 
        (WoodType::Pine, "oak_tree", 6), // TODO assets
        (WoodType::Birch, "oak_tree", 6), // TODO assets
        (WoodType::Maple, "maple_tree", 3),
        (WoodType::Spruce, "spruce_tree", 5),
        (WoodType::Cedar, "cedar_tree", 4),
    ];

    for (wood_type, name, count) in &wood_types {
        let mut variations = Vec::new();
        
        for i in 1..=*count {
            let path = format!("sprites/trees/{}_{:02}.png", name, i);
            variations.push(asset_server.load(&path));
        }
        
        atlas.sprites.insert(*wood_type, variations);
    }

    commands.insert_resource(atlas);
    tracing::info!("✓ Tree atlas loaded");
}
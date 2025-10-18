use super::atlas::TreeAtlas;
use super::components::*;
use crate::rendering::components::HexTile;
use crate::rendering::hex_config::HexConfig;
use crate::rendering::lod::{FrustumCullable, LodLevel};
use crate::state::WorldCache;
use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use shared::types::buildings::{Building, BuildingCategory, TreeData, WoodType};

/// Spawn les bâtiments visuels
pub fn spawn_building_visuals(
    mut commands: Commands,
    world_cache: Res<WorldCache>,
    tree_atlas: Res<TreeAtlas>,
    hex_config: Res<HexConfig>,
    existing: Query<&HexTile, With<BuildingVisuals>>,
) {
    let existing_coords: std::collections::HashSet<_> = existing.iter().map(|h| h.coord).collect();

    for chunk in world_cache.chunks() {
        for building_data in &chunk.buildings {
            let building = &building_data.building;

            if existing_coords.contains(&building.coord) {
                continue;
            }

            if building.building_type.category != BuildingCategory::Natural {
                continue;
            }

            let wood_type = parse_wood_type(&building.building_type.variant);
            let variations = match tree_atlas.get_variations(wood_type) {
                Some(v) => v,
                None => {
                    tracing::warn!("No sprites for {:?}", wood_type);
                    continue;
                }
            };

            // Position hex
            let hex = building.coord.to_hex();
            let world_pos = hex_config.layout.hex_to_world_pos(hex);

            // Seed deterministe par building ID
            let mut rng = rand::rngs::StdRng::seed_from_u64(building.id);

            // Select random sprite
            let sprite_idx = rng.random_range(0..variations.len());
            let image = variations[sprite_idx].clone();

            // Scale variation
            let scale_var = rng.random_range(0.9..1.1);
            let scale = 1.0 * scale_var;

            // Random flip (50%)
            let flip_x = rng.random_bool(0.5);

            // Spawn main sprite
            let entity = commands
                .spawn((
                    Sprite {
                        image: image.clone(),
                        custom_size: Some(Vec2::splat(96.0 * scale)),
                        flip_x,
                        ..default()
                    },
                    Transform::from_translation(world_pos.extend(1.0)),
                    GlobalTransform::default(),
                    Visibility::default(),
                    HexTile {
                        coord: building.coord,
                        chunk_id: chunk.id,
                    },
                    BuildingVisuals {
                        sprite_index: sprite_idx,
                        variation: 0,
                        scale,
                        z_offset: 1.0,
                    },
                    LodLevel::High,
                    FrustumCullable::new(48.0),
                ))
                .id();

            if let Some(tree_data) = &building_data.tree_data {
                spawn_density_sprites(
                    &mut commands,
                    entity,
                    building.id,
                    world_pos,
                    tree_data.density,
                    variations,
                );
            }
        }
    }
}

/// Spawn visuel d'un arbre
// fn spawn_tree_visual(
//     commands: &mut Commands,
//     building: &Building,
//     tree_data: &TreeData,
//     world_pos: Vec2,
//     building_materials: &BuildingMaterials,
//     biome_materials: &BiomeMaterials,
// ) {
//     // Variation basée sur ID (0-7)
//     let variation = (building.id % 8) as u8;

//     // Scale basé sur âge
//     let scale = tree_data.age.scale_multiplier();

//     // Nombre de sprites basé sur densité
//     let sprite_count = (tree_data.density * 5.0).ceil() as u8;

//     // Matériau basé sur type de bois
//     let material = building_materials.get_material(tree_data.wood_type);

//     // Entity parent
//     let parent = commands
//         .spawn((
//             BuildingEntity {
//                 id: building.id,
//                 coord: building.coord,
//                 building_type: building.building_type.clone(),
//             },
//             BuildingVisuals {
//                 sprite_index: 0,
//                 variation,
//                 scale,
//                 z_offset: 1.0,
//             },
//             VisualDensity {
//                 base_sprites: sprite_count,
//                 scatter_pattern: building.id,
//             },
//             BuildingAnimation::None,
//             Transform::from_translation(world_pos.extend(1.0)),
//         ))
//         .id();

//     // Spawn sprites multiples pour densité
//     spawn_density_sprites(
//         commands,
//         parent,
//         sprite_count,
//         building.id,
//         scale,
//         material,
//         biome_materials,
//     );
// }

/// Spawn plusieurs sprites pour effet de densité
fn spawn_density_sprites(
    commands: &mut Commands,
    parent: Entity,
    seed: u64,
    base_pos: Vec2,
    density: f32,
    variations: &[Handle<Image>],
) {
    let sprite_count = (density * 5.0).ceil() as usize;
    if sprite_count <= 1 {
        return;
    }

    let mut rng = rand::rngs::StdRng::seed_from_u64(seed.wrapping_add(1));

    for i in 0..sprite_count.saturating_sub(1) {
        let offset = Vec2::new(rng.random_range(-15.0..15.0), rng.random_range(-12.0..12.0));

        let sprite_idx = rng.random_range(0..variations.len());
        let image = variations[sprite_idx].clone();
        let scale = rng.random_range(0.7..1.1);
        let flip_x = rng.random_bool(0.5);

        commands.spawn((
            Sprite {
                image,
                custom_size: Some(Vec2::splat(96.0 * scale)),
                flip_x,
                ..default()
            },
            Transform::from_translation((base_pos + offset).extend(1.0 + i as f32 * 0.01)),
        ));
    }
}

/// Spawn bâtiment générique (placeholder)
fn spawn_generic_building(commands: &mut Commands, building: &Building, world_pos: Vec2) {
    commands.spawn((
        BuildingEntity {
            id: building.id,
            coord: building.coord,
            building_type: building.building_type.clone(),
        },
        Transform::from_translation(world_pos.extend(1.0)),
    ));
}

/// Despawn bâtiments des chunks déchargés
pub fn despawn_unloaded_buildings(
    mut commands: Commands,
    world_cache: Res<WorldCache>,
    buildings: Query<(Entity, &HexTile), With<BuildingVisuals>>,
) {
    let loaded_chunks: std::collections::HashSet<_> = world_cache.chunks().map(|c| c.id).collect();

    for (entity, building) in &buildings {
        if !loaded_chunks.contains(&building.chunk_id) {
            commands.entity(entity).try_despawn();
        }
    }
}

fn parse_wood_type(variant: &str) -> WoodType {
    match variant.to_lowercase().as_str() {
        s if s.contains("oak") => WoodType::Oak,
        s if s.contains("pine") => WoodType::Pine,
        s if s.contains("birch") => WoodType::Birch,
        s if s.contains("maple") => WoodType::Maple,
        s if s.contains("spruce") => WoodType::Spruce,
        s if s.contains("cedar") => WoodType::Cedar,
        _ => WoodType::Oak,
    }
}

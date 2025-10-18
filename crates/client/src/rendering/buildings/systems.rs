use bevy::prelude::*;
use crate::state::WorldCache;
use super::{components::*, materials::BuildingMaterials};
use crate::rendering::{HexConfig, BiomeMaterials};
use shared::types::*;

/// Setup des matériaux bâtiments
pub fn setup_building_materials(
    mut commands: Commands,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    let building_materials = BuildingMaterials::create(materials);
    commands.insert_resource(building_materials);
    tracing::info!("✓ Building materials créés");
}

/// Spawn les bâtiments visuels
pub fn spawn_building_visuals(
    mut commands: Commands,
    world_cache: Res<WorldCache>,
    building_materials: Res<BuildingMaterials>,
    biome_materials: Res<BiomeMaterials>,
    hex_config: Res<HexConfig>,
    existing: Query<&BuildingEntity>,
) {
    let existing_ids: std::collections::HashSet<_> = 
        existing.iter().map(|b| b.id).collect();
    
    for chunk in world_cache.chunks() {
        for building_data in &chunk.buildings {
            let building = &building_data.building;
            
            if existing_ids.contains(&building.id) {
                continue;
            }
            
            // Position hex
            let hex = building.coord.to_hex();
            let world_pos = hex_config.layout.hex_to_world_pos(hex);
            
            // Données spécifiques arbres
            if let Some(ref tree_data) = building_data.tree_data {
                spawn_tree_visual(
                    &mut commands,
                    building,
                    tree_data,
                    world_pos,
                    &building_materials,
                    &biome_materials,
                );
            } else {
                // Autres bâtiments (futur)
                spawn_generic_building(
                    &mut commands,
                    building,
                    world_pos,
                );
            }
        }
    }
}

/// Spawn visuel d'un arbre
fn spawn_tree_visual(
    commands: &mut Commands,
    building: &Building,
    tree_data: &TreeData,
    world_pos: Vec2,
    building_materials: &BuildingMaterials,
    biome_materials: &BiomeMaterials,
) {
    // Variation basée sur ID (0-7)
    let variation = (building.id % 8) as u8;
    
    // Scale basé sur âge
    let scale = tree_data.age.scale_multiplier();
    
    // Nombre de sprites basé sur densité
    let sprite_count = (tree_data.density * 5.0).ceil() as u8;
    
    // Matériau basé sur type de bois
    let material = building_materials.get_material(tree_data.wood_type);
    
    // Entity parent
    let parent = commands.spawn((
        BuildingEntity {
            id: building.id,
            coord: building.coord,
            building_type: building.building_type.clone(),
        },
        BuildingVisuals {
            sprite_index: 0,
            variation,
            scale,
            z_offset: 1.0,
        },
        VisualDensity {
            base_sprites: sprite_count,
            scatter_pattern: building.id,
        },
        BuildingAnimation::None,
        Transform::from_translation(world_pos.extend(1.0)),
    )).id();
    
    // Spawn sprites multiples pour densité
    spawn_density_sprites(
        commands,
        parent,
        sprite_count,
        building.id,
        scale,
        material,
        biome_materials,
    );
}

/// Spawn plusieurs sprites pour effet de densité
fn spawn_density_sprites(
    commands: &mut Commands,
    parent: Entity,
    count: u8,
    seed: u64,
    scale: f32,
    material: Handle<ColorMaterial>,
    biome_materials: &BiomeMaterials,
) {
    use rand::{SeedableRng, Rng};
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    
    for i in 0..count {
        // Offset aléatoire dans l'hexagone
        let offset_x = rng.random_range(-15.0..15.0);
        let offset_y = rng.random_range(-10.0..10.0);
        
        // Variation de scale
        let sprite_scale = scale * rng.random_range(0.8..1.2);
        
        commands.spawn((
            Mesh2d(biome_materials.hex_mesh.clone()),
            MeshMaterial2d(material.clone()),
            Transform::from_translation(Vec3::new(offset_x, offset_y, i as f32 * 0.01))
                .with_scale(Vec3::splat(sprite_scale * 0.3)), // Plus petit pour densité
        )).set_parent_in_place(parent);
    }
}

/// Spawn bâtiment générique (placeholder)
fn spawn_generic_building(
    commands: &mut Commands,
    building: &Building,
    world_pos: Vec2,
) {
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
    buildings: Query<(Entity, &BuildingEntity)>,
) {
    let loaded_coords: std::collections::HashSet<_> = world_cache
        .chunks()
        .flat_map(|c| c.buildings.iter().map(|b| b.building.coord))
        .collect();
    
    for (entity, building) in &buildings {
        if !loaded_coords.contains(&building.coord) {
            commands.entity(entity).despawn_children();
        }
    }
}
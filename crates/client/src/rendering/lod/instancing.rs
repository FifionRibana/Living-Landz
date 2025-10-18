use bevy::prelude::*;
use std::collections::HashMap;
use crate::rendering::components::HexVisuals;

/// Instance data pour batch rendering
#[derive(Clone, Copy)]
pub struct InstanceData {
    pub transform: Transform,
    pub color: Color,
    pub texture_index: usize,
}

/// Batch par sprite type (LOD1 instancing)
#[derive(Resource)]
pub struct InstancedBatches {
    pub batches: HashMap<usize, Vec<InstanceData>>,
}

impl Default for InstancedBatches {
    fn default() -> Self {
        Self {
            batches: HashMap::new(),
        }
    }
}

impl InstancedBatches {
    pub fn clear(&mut self) {
        self.batches.clear();
    }

    pub fn add_instance(&mut self, texture_index: usize, data: InstanceData) {
        self.batches.entry(texture_index).or_default().push(data);
    }

    pub fn batch_count(&self) -> usize {
        self.batches.values().map(|v| v.len()).sum()
    }

    pub fn draw_call_count(&self) -> usize {
        self.batches.len()
    }
}

/// Prepare instances pour LOD1 (Medium)
pub fn prepare_instanced_batches(
    mut batches: ResMut<InstancedBatches>,
    query: Query<(&Transform, &HexVisuals), Changed<Transform>>,
) {
    batches.clear();

    for (transform, visuals) in query.iter() {
        let data = InstanceData {
            transform: *transform,
            color: visuals.tint,
            texture_index: visuals.texture_index,
        };
        batches.add_instance(visuals.texture_index, data);
    }
}

/// Debug: log batch stats
pub fn debug_batch_stats(
    batches: Res<InstancedBatches>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyB) {
        let total_instances = batches.batch_count();
        let draw_calls = batches.draw_call_count();

        tracing::info!(
            "=== BATCH STATS ===",
        );
        tracing::info!("Total instances: {}", total_instances);
        tracing::info!("Draw calls: {}", draw_calls);
        tracing::info!("Avg per batch: {:.1}", 
            total_instances as f32 / draw_calls as f32
        );

        // Top batches
        let mut top: Vec<_> = batches.batches.iter()
            .map(|(idx, v)| (*idx, v.len()))
            .collect();
        top.sort_by_key(|(_idx, len)| std::cmp::Reverse(*len));

        for (idx, len) in top.iter().take(5) {
            tracing::info!("  Texture {}: {} instances", idx, len);
        }
    }
}

/// Render instanced batch (simplified - actual implementation depends on your rendering backend)
/// NOTE: Ceci est pseudo-code. L'implémentation réelle nécessite:
/// - Custom render pipeline OR
/// - Bevy instancing extension OR  
/// - Multiple entity spawn (simplest pour Bevy actuel)
pub fn spawn_instanced_sprites(
    mut commands: Commands,
    batches: Res<InstancedBatches>,
    query: Query<Entity, With<super::components::LodLevel>>,
) {
    // Clear old instances
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }

    // NOTE: Implémentation full instancing nécessite custom shader
    // Pour MVP: créer entities individuellement reste OK si culling actif
    // (sera optimisé en phase 4 si perf insuffisante)
}

/// LOD0 (Low zoom): Merge all visible hexes en 1-2 sprites
pub fn prepare_merged_sprite_lod0(
    frustum: Res<super::components::ViewFrustum>,
    query: Query<(&Transform, &HexVisuals), With<super::components::LodLevel>>,
) -> Option<(Transform, Color)> {
    let mut total_pos = Vec2::ZERO;
    let mut total_tint = Color::WHITE;
    let mut count = 0u32;

    for (transform, visuals) in query.iter() {
        let pos = transform.translation.truncate();
        if frustum.contains(pos, 50.0) {
            total_pos += pos;
            total_tint = Color::srgb(
                (total_tint.to_srgba().red + visuals.tint.to_srgba().red) / 2.0,
                (total_tint.to_srgba().green + visuals.tint.to_srgba().green) / 2.0,
                (total_tint.to_srgba().blue + visuals.tint.to_srgba().blue) / 2.0,
            );
            count += 1;
        }
    }

    if count == 0 {
        return None;
    }

    let avg_pos = total_pos / count as f32;
    Some((
        Transform::from_translation(avg_pos.extend(0.0)),
        total_tint,
    ))
}
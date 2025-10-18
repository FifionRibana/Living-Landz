use bevy::prelude::*;
use crate::camera::MainCamera;
use super::components::*;

/// Calcule frustum view depuis caméra
pub fn update_view_frustum(
    camera: Query<(&Transform, &Projection), With<MainCamera>>,
    mut frustum: ResMut<ViewFrustum>,
) {
    let Ok((transform, projection)) = camera.single() else {
        return;
    };

    let zoom = if let Projection::Orthographic(ortho) = projection {
        ortho.scale
    } else {
        1.0
    };

    // View distance augmente avec zoom (voir plus loin quand dézoomé)
    let view_distance = 2000.0 * zoom;

    frustum.center = transform.translation.truncate();
    frustum.radius = view_distance;
}

/// Rebuiild spatial grid chaque frame
pub fn rebuild_spatial_grid(
    mut grid: ResMut<SpatialGrid>,
    query: Query<(Entity, &Transform), With<FrustumCullable>>,
) {
    grid.clear();

    for (entity, transform) in query.iter() {
        let pos = transform.translation.truncate();
        grid.insert(pos, entity);
    }
}

/// Frustum culling: show/hide sprites
pub fn frustum_culling(
    frustum: Res<ViewFrustum>,
    grid: Res<SpatialGrid>,
    mut query: Query<(&Transform, &FrustumCullable, &mut Visibility)>,
) {
    // Obtenir cellules visibles
    let visible_cells = grid.cells_in_view(frustum.center, frustum.radius);
    let visible_entities: std::collections::HashSet<_> = visible_cells
        .iter()
        .flat_map(|cell| grid.grid.get(cell).cloned().unwrap_or_default())
        .collect();

    // Hide entities hors frustum
    for (transform, cullable, mut visibility) in query.iter_mut() {
        let pos = transform.translation.truncate();
        let is_visible = frustum.contains(pos, cullable.bounds.width());

        if is_visible && !visible_entities.is_empty() {
            *visibility = Visibility::Visible;
        } else if !is_visible {
            *visibility = Visibility::Hidden;
        }
    }
}

/// LOD switching basé zoom caméra
pub fn update_lod_levels(
    camera: Query<&Projection, With<MainCamera>>,
    mut query: Query<(&mut LodState, &mut LodLevel)>,
) {
    let Ok(projection) = camera.single() else {
        return;
    };

    let zoom = if let Projection::Orthographic(ortho) = projection {
        ortho.scale
    } else {
        1.0
    };

    let new_lod = LodLevel::from_zoom(zoom);

    for (mut state, mut current_lod) in query.iter_mut() {
        if state.current != new_lod {
            state.previous = state.current;
            state.current = new_lod;
            state.transition_timer = 0.0;  // Start transition
            *current_lod = new_lod;
        }
    }
}

/// Smooth transition entre LOD (optionnel, améliore UX)
pub fn transition_lod_visibility(
    time: Res<Time>,
    mut query: Query<(&mut LodState, &mut Sprite)>,
) {
    const TRANSITION_DURATION: f32 = 0.2;

    for (mut state, mut sprite) in query.iter_mut() {
        if state.transition_timer < 1.0 {
            state.transition_timer += time.delta_secs() / TRANSITION_DURATION;
            state.transition_timer = state.transition_timer.min(1.0);

            // Fade smooth
            let alpha = state.transition_timer;
            if let Color::Srgba(ref mut srgba) = sprite.color {
                srgba.alpha = alpha;
            }
        }
    }
}

/// Debug: afficher grid cells
pub fn debug_spatial_grid(
    grid: Res<SpatialGrid>,
    frustum: Res<ViewFrustum>,
    keys: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos,
) {
    if !keys.pressed(KeyCode::KeyG) {
        return;
    }

    let cells = grid.cells_in_view(frustum.center, frustum.radius);

    for (cx, cy) in cells {
        let min = Vec2::new(
            cx as f32 * grid.cell_size,
            cy as f32 * grid.cell_size,
        );
        let max = min + Vec2::splat(grid.cell_size);

        gizmos.rect_2d(
            (min + max) / 2.0,
            max - min,
            Color::srgba(0.0, 1.0, 0.0, 0.2),
        );

        // Entity count
        // if let Some(entities) = grid.grid.get(&(cx, cy)) {
        //     if !entities.is_empty() {
        //         gizmos.text_2d(
        //             format!("{}", entities.len()),
        //             (min + max) / 2.0,
        //             TextFont::default(),
        //             Color::WHITE,
        //         );
        //     }
        // }
    }
}
use bevy::prelude::*;
use shared::{BiomeType, HexCoord, ChunkId};

/// Niveaux de détail actifs
#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum LodLevel {
    /// Zoom < 0.5: Texture merged continent (1 sprite)
    Low,
    /// Zoom 0.5-2.0: Instanced sprites (batch draw)
    Medium,
    /// Zoom > 2.0: Individual sprites (full detail)
    High,
}

impl LodLevel {
    pub fn from_zoom(zoom: f32) -> Self {
        match zoom {
            z if z < 0.5 => LodLevel::High,
            z if z < 2.0 => LodLevel::Medium,
            _ => LodLevel::Low,
        }
    }
}

/// Marqueur pour frustum culling
#[derive(Component)]
pub struct FrustumCullable {
    pub last_visible: bool,
    pub bounds: Rect,  // AABB locale (hex size)
}

impl FrustumCullable {
    pub fn new(hex_radius: f32) -> Self {
        Self {
            last_visible: true,
            bounds: Rect::from_center_half_size(
                Vec2::ZERO,
                Vec2::splat(hex_radius * 1.5),
            ),
        }
    }
}

/// Grid spatial pour culling rapide
#[derive(Resource)]
pub struct SpatialGrid {
    pub cell_size: f32,
    pub grid: std::collections::HashMap<(i32, i32), Vec<Entity>>,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            cell_size,
            grid: Default::default(),
        }
    }

    pub fn cell_key(&self, pos: Vec2) -> (i32, i32) {
        let cell_size = self.cell_size;
        (
            (pos.x / cell_size).floor() as i32,
            (pos.y / cell_size).floor() as i32,
        )
    }

    pub fn cells_in_view(&self, camera_pos: Vec2, view_radius: f32) -> Vec<(i32, i32)> {
        let half_cells = (view_radius / self.cell_size).ceil() as i32;
        let (cx, cy) = self.cell_key(camera_pos);

        let mut cells = Vec::new();
        for dx in -half_cells..=half_cells {
            for dy in -half_cells..=half_cells {
                cells.push((cx + dx, cy + dy));
            }
        }
        cells
    }

    pub fn insert(&mut self, pos: Vec2, entity: Entity) {
        let key = self.cell_key(pos);
        self.grid.entry(key).or_default().push(entity);
    }

    pub fn clear(&mut self) {
        self.grid.clear();
    }
}

/// Cache LOD state pour transitions smooth
#[derive(Component)]
pub struct LodState {
    pub current: LodLevel,
    pub previous: LodLevel,
    pub transition_timer: f32,  // 0.0 = no transition, 1.0 = complete
}

impl Default for LodState {
    fn default() -> Self {
        Self {
            current: LodLevel::Medium,
            previous: LodLevel::Medium,
            transition_timer: 1.0,
        }
    }
}

/// Culling frustum (simplified AABB)
#[derive(Resource)]
pub struct ViewFrustum {
    pub center: Vec2,
    pub radius: f32,  // View distance
}

impl ViewFrustum {
    pub fn contains(&self, pos: Vec2, size: f32) -> bool {
        let dist = self.center.distance(pos);
        dist < self.radius + size
    }
}
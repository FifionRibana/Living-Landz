use bevy::prelude::*;
use hexx::*;

#[derive(Resource)]
pub struct HexRenderer {
    pub layout: HexLayout,
    pub hex_size: f32,
}

impl Default for HexRenderer {
    fn default() -> Self {
        Self {
            layout: HexLayout {
                hex_size: Vec2::splat(32.0),
                orientation: HexOrientation::Flat,
                ..default()
            },
            hex_size: 32.0,
        }
    }
}

pub fn render_hex_grid(
    mut gizmos: Gizmos,
    hex_renderer: Res<HexRenderer>,
    camera: Query<&Transform, With<Camera>>,
    world_cache: Res<WorldCache>,
) {
    let camera_transform = camera.single();
    
    // Calculer hexagones visibles
    let visible_hexes = calculate_visible_hexes(
        camera_transform,
        &hex_renderer,
    );
    
    // Dessiner chaque hex
    for hex_coord in visible_hexes {
        let world_pos = hex_renderer.layout.hex_to_world_pos(hex_coord.to_hex());
        
        // Récupérer données tile depuis cache
        if let Some(tile) = world_cache.get_tile(hex_coord) {
            let color = biome_color(tile.biome);
            draw_hex(&mut gizmos, world_pos, hex_renderer.hex_size, color);
        }
    }
}

fn draw_hex(gizmos: &mut Gizmos, center: Vec2, size: f32, color: Color) {
    let points = hex_corners(center, size);
    for i in 0..6 {
        let start = points[i];
        let end = points[(i + 1) % 6];
        gizmos.line_2d(start, end, color);
    }
}

fn hex_corners(center: Vec2, size: f32) -> [Vec2; 6] {
    let mut corners = [Vec2::ZERO; 6];
    for i in 0..6 {
        let angle = std::f32::consts::PI / 3.0 * i as f32;
        corners[i] = center + Vec2::new(
            size * angle.cos(),
            size * angle.sin(),
        );
    }
    corners
}
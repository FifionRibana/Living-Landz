use bevy::prelude::*;

use crate::state::WorldCache;
use shared::{BiomeType, HexCoord};

const HEX_SIZE: f32 = 32.0;

pub fn render_hex_grid(
    mut gizmos: Gizmos,
    camera: Query<(&Transform, &Projection), With<Camera>>,
    world_cache: Res<WorldCache>,
) {
    let Ok((camera_transform, projection)) = camera.single() else {
        return;
    };

    let scale = if let Projection::Orthographic(ortho) = projection {
        ortho.scale
    } else {
        1.0
    };

    let view_distance = 2000.0 * scale;

    for chunk in world_cache.chunks() {
        for tile in &chunk.tiles {
            let world_pos = hex_to_world(tile.coord);

            let dist = camera_transform.translation.truncate().distance(world_pos);
            if dist > view_distance {
                continue;
            }
            let color = biome_color(tile.biome);
            draw_hex(&mut gizmos, world_pos, HEX_SIZE, color);
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
        corners[i] = center + Vec2::new(size * angle.cos(), size * angle.sin());
    }
    corners
}

pub fn hex_to_world(coord: HexCoord) -> Vec2 {
    let sqrt3 = 1.732050808;
    Vec2::new(
        HEX_SIZE * (sqrt3 * coord.q as f32 + sqrt3 / 2.0 * coord.r as f32),
        HEX_SIZE * (3.0 / 2.0 * coord.r as f32),
    )
}

fn biome_color(biome: BiomeType) -> Color {
    match biome {
        BiomeType::Ocean => Color::srgb(0.0, 0.3, 0.8),
        BiomeType::DeepOcean => Color::srgb(0.0, 0.1, 0.5),
        BiomeType::Coast => Color::srgb(0.5, 0.7, 0.9),
        BiomeType::Beach => Color::srgb(0.9, 0.85, 0.7),
        BiomeType::Lake => Color::srgb(0.2, 0.5, 0.9),
        BiomeType::Grassland => Color::srgb(0.2, 0.8, 0.2),
        BiomeType::Forest => Color::srgb(0.0, 0.5, 0.0),
        BiomeType::DenseForest => Color::srgb(0.0, 0.3, 0.0),
        BiomeType::Mountain => Color::srgb(0.5, 0.5, 0.5),
        BiomeType::HighMountain => Color::srgb(0.8, 0.8, 0.8),
        BiomeType::Desert => Color::srgb(0.9, 0.8, 0.5),
        BiomeType::Tundra => Color::srgb(0.8, 0.9, 0.9),
        BiomeType::Taiga => Color::srgb(0.3, 0.5, 0.4),
        BiomeType::Swamp => Color::srgb(0.4, 0.5, 0.3),
        BiomeType::Ice => Color::srgb(1.0, 1.0, 1.0),
    }
}

/*
fn render_hex_grid(
    mut gizmos: Gizmos,
    world_cache: Res<WorldCache>,
    camera_query: Query<(&Transform, &Projection), With<MainCamera>>,
) {
    let Ok((camera_transform, projection)) = camera_query.single() else {
        return;
    };

    let scale = if let Projection::Orthographic(ortho) = projection {
        ortho.scale
    } else {
        1.0
    };

    let hex_size = 20.0;
    let mut rendered_count = 0;

    for (coord, tile) in world_cache.tiles.iter() {
        let world_pos = hex_to_world(*coord, hex_size);

        let dist = camera_transform.translation.truncate().distance(world_pos);
        if dist > 3000.0 * scale {
            continue;
        }

        let color = biome_color(tile.biome);
        draw_hexagon(&mut gizmos, world_pos, hex_size, color);
        rendered_count += 1;
    }

    static mut FIRST_FRAME: bool = true;
    unsafe {
        if FIRST_FRAME {
            println!("Rendering {} hexagons", rendered_count);
            FIRST_FRAME = false;
        }
    }

}
*/

use bevy::prelude::*;
use std::collections::HashMap;
use log::info;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Living Landz - Prototype".to_string(),
                resolution: (1280, 720).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<WorldCache>()
        .init_resource::<CameraSettings>()
        .add_systems(Startup, (setup_camera, setup_test_world)) // ← Ajout
        .add_systems(Update, (
            camera_movement,
            camera_zoom,
            render_hex_grid,
            mouse_pick_hex,
            debug_info,
        ))
        .run();
}

#[derive(Resource, Default)]
struct WorldCache {
    tiles: HashMap<shared::HexCoord, shared::TileData>,
}

#[derive(Resource)]
struct CameraSettings {
    speed: f32,
    zoom_speed: f32,
    min_zoom: f32,
    max_zoom: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            speed: 500.0,
            zoom_speed: 0.1,
            min_zoom: 0.5,
            max_zoom: 3.0,
        }
    }
}

#[derive(Component)]
struct MainCamera;

fn setup_camera(mut commands: Commands) {
    println!("Setup camera called!");
    commands.spawn((
        Camera2d,
        MainCamera,
    ));
}

fn setup_test_world(mut world_cache: ResMut<WorldCache>) {
    println!("Setup test world called!");
    let generator = hex_grid::WorldGenerator::new(12345);
    
    for cx in -2..=2 {
        for cy in -2..=2 {
            let chunk_id = shared::ChunkId { x: cx, y: cy };
            let tiles = generator.generate_chunk(chunk_id);
            
            for tile in tiles {
                world_cache.tiles.insert(tile.coord, tile);
            }
        }
    }
    
    println!("Loaded {} tiles", world_cache.tiles.len());
    info!("Loaded {} tiles", world_cache.tiles.len());
}

// ← NOUVEAU : Système de test avec des sprites colorés
fn spawn_test_sprites(mut commands: Commands) {
    println!("Spawning test sprites...");
    
    // Spawn plusieurs sprites colorés pour test
    for i in -5..=5 {
        for j in -5..=5 {
            let x = i as f32 * 64.0;
            let y = j as f32 * 64.0;
            
            commands.spawn((
                Sprite {
                    color: Color::srgb(
                        ((i + 5) as f32) / 10.0,
                        ((j + 5) as f32) / 10.0,
                        0.5,
                    ),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
    
    println!("Spawned test sprites!");
}

fn camera_movement(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<CameraSettings>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    let Ok(mut transform) = camera_query.single_mut() else {
        return;
    };
    
    let speed = settings.speed * time.delta_secs();

    if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
        transform.translation.y += speed;
    }
    if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
        transform.translation.y -= speed;
    }
    if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
        transform.translation.x -= speed;
    }
    if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
        transform.translation.x += speed;
    }
}

fn camera_zoom(
    mut scroll_events: MessageReader<bevy::input::mouse::MouseWheel>,
    settings: Res<CameraSettings>,
    mut camera_query: Query<&mut Projection, With<MainCamera>>,
) {
    let Ok(mut projection) = camera_query.single_mut() else {
        return;
    };

    for event in scroll_events.read() {
        if let Projection::Orthographic(ref mut ortho) = projection.as_mut() {
            ortho.scale -= event.y * settings.zoom_speed * 0.1;
            ortho.scale = ortho.scale.clamp(settings.min_zoom, settings.max_zoom);
            println!("Zoom: {}", ortho.scale);
        }
    }
}

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

fn draw_hexagon(gizmos: &mut Gizmos, center: Vec2, size: f32, color: Color) {
    let corners = hex_corners(center, size);
    for i in 0..6 {
        gizmos.line_2d(corners[i], corners[(i + 1) % 6], color);
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

fn hex_to_world(coord: shared::HexCoord, hex_size: f32) -> Vec2 {
    let sqrt3 = 1.732050808;
    Vec2::new(
        hex_size * (sqrt3 * coord.q as f32 + sqrt3 / 2.0 * coord.r as f32),
        hex_size * (3.0 / 2.0 * coord.r as f32),
    )
}

fn biome_color(biome: shared::BiomeType) -> Color {
    match biome {
        shared::BiomeType::Ocean => Color::srgb(0.0, 0.3, 0.8),
        shared::BiomeType::Coast => Color::srgb(0.7, 0.7, 0.3),
        shared::BiomeType::Grassland => Color::srgb(0.2, 0.8, 0.2),
        shared::BiomeType::Forest => Color::srgb(0.0, 0.5, 0.0),
        shared::BiomeType::Mountain => Color::srgb(0.5, 0.5, 0.5),
        shared::BiomeType::Desert => Color::srgb(0.9, 0.8, 0.5),
        shared::BiomeType::Tundra => Color::srgb(0.8, 0.9, 0.9),
        shared::BiomeType::Ice => Color::srgb(1.0, 1.0, 1.0),
    }
}

fn mouse_pick_hex(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    if buttons.just_pressed(MouseButton::Left) {
        let Ok(window) = windows.single() else {
            return;
        };
        let Ok((camera, camera_transform)) = camera_query.single() else {
            return;
        };

        if let Some(cursor_pos) = window.cursor_position() {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                println!("Clicked at world position: {:?}", world_pos);
                info!("Clicked at world position: {:?}", world_pos);
            }
        }
    }
}

fn debug_info(
    camera_query: Query<(&Transform, &Projection), With<MainCamera>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyI) {
        if let Ok((transform, projection)) = camera_query.single() {
            let scale = if let Projection::Orthographic(ortho) = projection {
                ortho.scale
            } else {
                1.0
            };
            
            println!("=== DEBUG INFO ===");
            println!("Camera position: {:?}", transform.translation);
            println!("Camera scale: {}", scale);
            println!("==================");
        }
    }
}
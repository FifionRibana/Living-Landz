use bevy::prelude::*;

fn main() {
    println!("Starting Bevy example...");
    
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Test Bevy".to_string(),
                resolution: (800, 600).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (debug, debug_entities))
        .run();
}

fn setup(mut commands: Commands) {
    println!("=== SETUP CALLED ===");
    
    // Caméra avec configuration explicite
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.9, 0.2, 0.3)), // Bleu foncé
            ..default()
        },
    ));
    
    // Plusieurs sprites de test
    // Sprite 1 : Gros carré rouge au centre
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::new(200.0, 200.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0), // Z = 1 pour être devant
    ));
    
    // Sprite 2 : Carré vert à gauche
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 1.0, 0.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(-200.0, 0.0, 1.0),
    ));
    
    // Sprite 3 : Carré bleu à droite
    commands.spawn((
        Sprite {
            color: Color::srgb(0.0, 0.0, 1.0),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        Transform::from_xyz(200.0, 0.0, 1.0),
    ));
    
    println!("=== SETUP COMPLETE - Spawned 3 sprites ===");
}

fn debug(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("Space pressed! If you see this but no sprites, there's a rendering issue.");
    }
}

fn debug_entities(
    query: Query<(Entity, &Transform, Option<&Sprite>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::KeyD) {
        println!("=== ENTITIES DEBUG ===");
        for (entity, transform, sprite) in query.iter() {
            println!("Entity {:?}:", entity);
            println!("  Position: {:?}", transform.translation);
            if let Some(sprite) = sprite {
                println!("  Sprite color: {:?}", sprite.color);
                println!("  Sprite size: {:?}", sprite.custom_size);
            }
        }
        println!("======================");
    }
}
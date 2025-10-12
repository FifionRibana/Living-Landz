use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, debug)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("=== SETUP ===");
    
    // Caméra
    commands.spawn(Camera2d);
    
    // Cercle rouge au centre
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(100.0))),
        MeshMaterial2d(materials.add(Color::srgb(1.0, 0.0, 0.0))),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    
    // Cercle vert à gauche
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 1.0, 0.0))),
        Transform::from_xyz(-200.0, 0.0, 0.0),
    ));
    
    // Rectangle bleu à droite
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(100.0, 150.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.0, 0.0, 1.0))),
        Transform::from_xyz(200.0, 0.0, 0.0),
    ));
    
    println!("=== SETUP COMPLETE ===");
}

fn debug(keys: Res<ButtonInput<KeyCode>>) {
    if keys.just_pressed(KeyCode::Space) {
        println!("Space pressed!");
    }
}
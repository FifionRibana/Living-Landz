use bevy::prelude::*;
use super::components::*;

/// Système de debug pour afficher les hexagones
pub fn debug_hex_rendering(
    keys: Res<ButtonInput<KeyCode>>,
    query: Query<(&HexTile, &HexVisuals, &Transform)>,
) {
    if !keys.just_pressed(KeyCode::KeyH) {
        return;
    }
    
    let count = query.iter().count();
    tracing::info!("=== HEX RENDERING DEBUG ===");
    tracing::info!("Total hexagones: {}", count);
    
    // Afficher quelques exemples
    for (i, (tile, visuals, transform)) in query.iter().enumerate().take(5) {
        tracing::info!(
            "Hex {}: coord=({}, {}), biome={:?}, pos=({:.1}, {:.1})",
            i,
            tile.coord.q,
            tile.coord.r,
            visuals.biome,
            transform.translation.x,
            transform.translation.y
        );
    }
}

// Ajouter au plugin:
// .add_systems(Update, debug_hex_rendering)

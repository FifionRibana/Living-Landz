
use bevy::prelude::*;
use crate::rendering::components::HexTile;
use crate::rendering::buildings::components::BuildingVisuals;

pub fn debug_entity_count(
    keys: Res<ButtonInput<KeyCode>>,
    hex_query: Query<&HexTile>,
    building_query: Query<&BuildingVisuals>,
    visible_query: Query<&Visibility>,
) {
    if !keys.just_pressed(KeyCode::KeyP) {
        return;
    }

    let hex_count = hex_query.iter().count();
    let building_count = building_query.iter().count();
    let visible_count = visible_query.iter().filter(|v| **v == Visibility::Visible).count();

    tracing::info!("=== PERFORMANCE DEBUG ===");
    tracing::info!("Total hex tiles: {}", hex_count);
    tracing::info!("Total buildings: {}", building_count);
    tracing::info!("Visible sprites: {}", visible_count);
    tracing::info!("Hidden sprites: {}", hex_count + building_count - visible_count);
    tracing::info!("Cull rate: {:.1}%", 
        (1.0 - visible_count as f32 / (hex_count + building_count) as f32) * 100.0
    );
}
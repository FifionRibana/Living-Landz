use super::atlas::setup_tree_atlas;
use super::systems::*;
use bevy::prelude::*;

pub struct BuildingPlugin;

impl Plugin for BuildingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_tree_atlas).add_systems(
            Update,
            (spawn_building_visuals, despawn_unloaded_buildings).chain(),
        );
    }
}

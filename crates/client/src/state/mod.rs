// =============================================================================
// STATE MODULE
// =============================================================================

pub mod world_cache;
pub mod streaming;

pub use world_cache::WorldCache;
pub use streaming::{StreamingConfig, request_chunks_around_camera, process_chunk_messages, unload_distant_chunks};

use bevy::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WorldCache>()
            .init_resource::<StreamingConfig>()
            .add_systems(Update, (
                request_chunks_around_camera,
                process_chunk_messages,
                unload_distant_chunks,
            ).chain());
    }
}
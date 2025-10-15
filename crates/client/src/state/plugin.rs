// =============================================================================
// STATE - Plugin
// =============================================================================

use bevy::prelude::*;

pub use super::connection::ConnectionStatus;
pub use super::streaming::{StreamingConfig, request_chunks_around_camera, process_chunk_messages, unload_distant_chunks};
pub use super::world_cache::WorldCache;


pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<WorldCache>()
            .init_resource::<StreamingConfig>()
            .init_resource::<ConnectionStatus>()
            .add_systems(Update, (
                process_chunk_messages,
                request_chunks_around_camera,
                unload_distant_chunks,
            ).chain());
    }
}
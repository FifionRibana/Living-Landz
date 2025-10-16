// =============================================================================
// RENDERING MODULE
// =============================================================================

pub mod atlas;
pub mod components;
pub mod coords;
pub mod hex_config;     // ← Renommé de hex_layout
pub mod hex_mesh;
pub mod hex_renderer;
pub mod lod;
pub mod plugin;
pub mod sprites;
pub mod systems;

pub use hex_config::HexConfig;
pub use plugin::RenderingPlugin;
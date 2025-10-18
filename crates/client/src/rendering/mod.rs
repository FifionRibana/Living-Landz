// =============================================================================
// RENDERING MODULE
// =============================================================================

pub mod atlas;
pub mod buildings;
pub mod components;
pub mod coords;
pub mod hex_config; // ← Renommé de hex_layout
pub mod hex_mesh;
pub mod hex_renderer;
pub mod lod;
pub mod plugin;
pub mod sprites;
pub mod systems;

pub use atlas::BiomeMaterials;
pub use buildings::BuildingMaterials;
pub use hex_config::HexConfig;
pub use plugin::RenderingPlugin;

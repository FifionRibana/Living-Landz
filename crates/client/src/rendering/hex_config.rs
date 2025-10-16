use bevy::prelude::*;
use hexx::{HexLayout, HexOrientation};

/// Resource wrapper pour la configuration hexagonale
#[derive(Resource, Clone)]
pub struct HexConfig {
    pub layout: HexLayout,
    pub hex_radius: f32,
}

impl Default for HexConfig {
    fn default() -> Self {
        Self::new(48.0)
    }
}

impl HexConfig {
    /// Crée une configuration avec un rayon donné
    pub fn new(radius: f32) -> Self {
        let layout = HexLayout::flat().with_hex_size(radius).with_scale(Vec2::new(1.*radius, 0.67 * radius));
        
        Self {
            layout,
            hex_radius: radius,
        }
    }
    
    /// Obtient le rayon des hexagones
    pub fn radius(&self) -> f32 {
        self.hex_radius
    }
}
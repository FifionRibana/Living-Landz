use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceType {
    // Matières premières
    Wood,
    Stone,
    Iron,
    Clay,
    
    // Agriculture
    Wheat,
    Bread,
    Meat,
    Fish,
    
    // Artisanat
    Tools,
    Weapons,
    Clothes,
    
    // Spécial
    Gold,
    // ... autres selon ton design
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub resource_type: ResourceType,
    pub quantity: f32,
    pub quality: u8,      // 0-100
    pub durability: f32,  // 0.0-1.0 (denrées périssables)
}

impl Resource {
    pub fn new(resource_type: ResourceType, quantity: f32, quality: u8) -> Self {
        Self {
            resource_type,
            quantity,
            quality,
            durability: 1.0,
        }
    }
}
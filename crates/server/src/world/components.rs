use legion::*;
use shared::types::*;

/// Position dans le monde
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub coord: HexCoord,
}

/// Bâtiment
#[derive(Clone, Debug)]
pub struct Building {
    pub building_type: BuildingType,
    pub owner_id: u64,
    pub construction_progress: f32, // 0.0 à 1.0
    pub health: u8,
}

/// Unité (personne)
#[derive(Clone, Debug)]
pub struct Unit {
    pub name: String,
    pub stats: Stats,
    pub profession: Option<Profession>,
    pub inventory: Vec<Resource>,
    pub home_city_id: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct Stats {
    pub strength: u8,
    pub agility: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
}

/// Ville
#[derive(Clone, Debug)]
pub struct City {
    pub name: String,
    pub owner_id: u64,
    pub population: u32,
    pub inventory: std::collections::HashMap<ResourceType, Vec<Resource>>,
}

/// Marqueur pour entités devant être envoyées au client
#[derive(Clone, Copy, Debug)]
pub struct NetworkedEntity;
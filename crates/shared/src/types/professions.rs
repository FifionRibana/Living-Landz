use serde::{Deserialize, Serialize};

// Ajouter Profession (si pas déjà présent)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Profession {
    Farmer,
    Miner,
    Blacksmith,
    Carpenter,
    // ...
}
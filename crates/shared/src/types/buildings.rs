use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BuildingType {
    Farm,
    House,
    Mine,
    Castle,
}
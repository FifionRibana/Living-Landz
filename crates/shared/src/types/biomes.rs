use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BiomeType {
    Ocean,
    DeepOcean,
    Coast,
    Beach,
    Grassland,
    Forest,
    DenseForest,
    Mountain,
    HighMountain,
    Desert,
    Tundra,
    Taiga,
    Swamp,
    Ice,
}

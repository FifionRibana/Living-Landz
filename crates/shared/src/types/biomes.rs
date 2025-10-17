use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BiomeType {
    Ocean,
    DeepOcean,
    Coast,
    Beach,
    Lake,
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

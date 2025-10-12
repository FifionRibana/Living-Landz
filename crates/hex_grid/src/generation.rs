use noise::{NoiseFn, Perlin};

pub struct WorldGenerator {
    seed: u64,
    altitude_noise: Perlin,
    moisture_noise: Perlin,
    temperature_noise: Perlin,
}

impl WorldGenerator {
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            altitude_noise: Perlin::new(seed as u32),
            moisture_noise: Perlin::new((seed + 1) as u32),
            temperature_noise: Perlin::new((seed + 2) as u32),
        }
    }
    
    pub fn generate_tile(&self, coord: HexCoord) -> Tile {
        let x = coord.q as f64 * 0.01;
        let y = coord.r as f64 * 0.01;
        
        let altitude = self.altitude_noise.get([x, y]);
        let moisture = self.moisture_noise.get([x * 2.0, y * 2.0]);
        let temperature = self.temperature_noise.get([x * 0.5, y * 0.5]);
        
        let biome = determine_biome(altitude, moisture, temperature);
        
        Tile {
            coord,
            biome,
            altitude: (altitude * 1000.0) as i16,
            quality: calculate_quality(altitude, moisture),
            has_water: altitude < -0.1,
            has_river: false, // À implémenter séparément
        }
    }
}

fn determine_biome(altitude: f64, moisture: f64, temperature: f64) -> BiomeType {
    if altitude < -0.1 {
        return BiomeType::Ocean;
    }
    
    if altitude < 0.0 {
        return BiomeType::Coast;
    }
    
    if altitude > 0.6 {
        return BiomeType::Mountain;
    }
    
    // Utilise moisture et temperature pour biomes terrestres
    match (moisture, temperature) {
        (m, t) if m > 0.3 && t > 0.3 => BiomeType::TropicalForest,
        (m, t) if m > 0.3 && t < -0.3 => BiomeType::Tundra,
        (m, _) if m > 0.3 => BiomeType::Forest,
        (m, t) if m < -0.3 && t > 0.3 => BiomeType::Desert,
        (m, t) if m < -0.3 && t < -0.3 => BiomeType::Ice,
        _ => BiomeType::Grassland,
    }
}
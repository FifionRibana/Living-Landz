
pub const HEX_SIZE: f32 = 48.0; // Changé de 32.0
pub const HEX_WIDTH: f32 = HEX_SIZE;
pub const HEX_HEIGHT: f32 = HEX_SIZE * 0.866; // Ratio isométrique (√3/2)


pub struct Constants {
    pub hex_size: f32,
    pub hex_height: f32
}

impl Default for Constants {
    fn default() -> Self {
        Self {
            hex_size: 48.0,
            hex_height: 41.57
        }
    }
}

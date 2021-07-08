pub enum LightingQuantity {
    PixelColour(RGBColour),
    Radiance(RGBColour),
}

pub struct RGBColour {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

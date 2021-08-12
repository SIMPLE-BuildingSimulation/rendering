use crate::Float;

pub struct Film {
    /// Contains the number of Pixels in Width and Height
    pub resolution: (usize, usize),
}

impl Film {
    /// return the Width/height ratio
    pub fn aspect_ratio(&self) -> Float {
        let (width, height) = self.resolution;
        width as Float / height as Float
    }
}

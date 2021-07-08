// pub enum LightingQuantity {
//     PixelColour(RGBSpectrum),
//     Radiance(RGBSpectrum),
// }

pub type Spectrum = RGBSpectrum;

#[derive(Clone, Copy)]
pub struct RGBSpectrum {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl RGBSpectrum {
    pub fn black() -> Self {
        Self {
            red: 0.,
            green: 0.,
            blue: 0.,
        }
    }
}

impl std::ops::Add for RGBSpectrum {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl std::ops::AddAssign for RGBSpectrum {
    fn add_assign(&mut self, other: Self) {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
    }
}

impl std::ops::Mul for RGBSpectrum {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl std::ops::MulAssign for RGBSpectrum {
    fn mul_assign(&mut self, other: Self) {
        self.red *= other.red;
        self.green *= other.green;
        self.blue *= other.blue;
    }
}

impl std::ops::Mul<f64> for RGBSpectrum {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl std::ops::MulAssign<f64> for RGBSpectrum {
    fn mul_assign(&mut self, other: f64) {
        self.red *= other;
        self.green *= other;
        self.blue *= other;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let c1 = RGBSpectrum {
            red: 1.23,
            green: 5.321,
            blue: 9.9719,
        };

        let c2 = RGBSpectrum {
            red: 21.23,
            green: 95.321,
            blue: 0.9719,
        };

        let c3 = c1 + c2;
        assert_eq!(c3.red, c1.red + c2.red);
        assert_eq!(c3.green, c1.green + c2.green);
        assert_eq!(c3.blue, c1.blue + c2.blue);
    }
    #[test]
    fn test_mul() {
        let c1 = RGBSpectrum {
            red: 1.23,
            green: 5.321,
            blue: 9.9719,
        };

        let c2 = RGBSpectrum {
            red: 21.23,
            green: 95.321,
            blue: 0.9719,
        };

        let c3 = c1 * c2;
        assert_eq!(c3.red, c1.red * c2.red);
        assert_eq!(c3.green, c1.green * c2.green);
        assert_eq!(c3.blue, c1.blue * c2.blue);
    }

    #[test]
    fn test_add_assign() {
        let c1 = RGBSpectrum {
            red: 1.23,
            green: 5.321,
            blue: 9.9719,
        };

        let c2 = RGBSpectrum {
            red: 21.23,
            green: 95.321,
            blue: 0.9719,
        };

        let mut c3 = c1;
        c3 += c2;

        assert_eq!(c3.red, c1.red + c2.red);
        assert_eq!(c3.green, c1.green + c2.green);
        assert_eq!(c3.blue, c1.blue + c2.blue);
    }

    #[test]
    fn test_mul_assign() {
        let c1 = RGBSpectrum {
            red: 1.23,
            green: 5.321,
            blue: 9.9719,
        };

        let c2 = RGBSpectrum {
            red: 21.23,
            green: 95.321,
            blue: 0.9719,
        };

        let mut c3 = c1;
        c3 *= c2;

        assert_eq!(c3.red, c1.red * c2.red);
        assert_eq!(c3.green, c1.green * c2.green);
        assert_eq!(c3.blue, c1.blue * c2.blue);
    }
}

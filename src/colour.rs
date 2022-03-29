/*
MIT License
Copyright (c) 2021 Germán Molina
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::Float;

pub type Spectrum = RGBSpectrum;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGBSpectrum {
    pub red: Float,
    pub green: Float,
    pub blue: Float,
}

impl matrix::OneZero for RGBSpectrum {
    fn one() -> Self {
        Self::gray(1.)
    }

    fn zero() -> Self {
        Self::black()
    }
}

impl RGBSpectrum {
    /// Creates a new Spectrum full of Zeroes
    pub fn black() -> Self {
        Self {
            red: 0.,
            green: 0.,
            blue: 0.,
        }
    }

    /// Creates a new Spectrum full of equal values `v`
    pub fn gray(v: Float) -> Self {
        Self {
            red: v,
            green: v,
            blue: v,
        }
    }

    /// Checks whether `Spectrum::black() == self`
    pub fn is_black(&self) -> bool {
        self.red < 1e-24 && self.green < 1e-24 && self.blue < 1e-24
    }

    /// Scales the chanels in order to make the
    /// radiance equals to 1
    pub fn normalize(&self) -> Self {
        *self / self.radiance()
    }

    /// Calculates a weighted average of RGB colours, returning
    /// a single value representing Radiance
    pub fn radiance(&self) -> Float {
        // self.red*47.9 + self.green*119.9 + self.blue*11.6
        self.red * 0.265 + self.green * 0.670 + self.blue * 0.065
    }

    /// Calculates a weighted average of RGB colours, returning
    /// a single value representing Radiance
    pub fn luminance(&self) -> Float {
        self.radiance() * Self::WHITE_EFFICACY
    }

    /// The standard Luminious Efficacy of equal white light energy
    /// as defined in Radiance
    pub const WHITE_EFFICACY: Float = 179.;

    /// Gets the maximum of the R, G, and B values
    pub fn max(&self) -> Float {
        let mut v = self.red;
        if self.green > v {
            v = self.green;
        }
        if self.blue > v {
            v = self.blue;
        }
        v
    }
}

impl std::fmt::Display for RGBSpectrum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.5} {:.5} {:.5}", self.red, self.green, self.blue)
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
impl std::ops::Sub for RGBSpectrum {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
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

impl std::ops::SubAssign for RGBSpectrum {
    fn sub_assign(&mut self, other: Self) {
        self.red -= other.red;
        self.green -= other.green;
        self.blue -= other.blue;
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

impl std::ops::Mul<Float> for RGBSpectrum {
    type Output = Self;

    fn mul(self, other: Float) -> Self {
        Self {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl std::ops::MulAssign<Float> for RGBSpectrum {
    fn mul_assign(&mut self, other: Float) {
        self.red *= other;
        self.green *= other;
        self.blue *= other;
    }
}

impl std::ops::Div<Float> for RGBSpectrum {
    type Output = Self;

    fn div(self, other: Float) -> Self {
        Self {
            red: self.red / other,
            green: self.green / other,
            blue: self.blue / other,
        }
    }
}

impl std::ops::Div for RGBSpectrum {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
        }
    }
}

impl std::ops::DivAssign<Float> for RGBSpectrum {
    fn div_assign(&mut self, other: Float) {
        self.red /= other;
        self.green /= other;
        self.blue /= other;
    }
}

impl std::ops::DivAssign for RGBSpectrum {
    fn div_assign(&mut self, other: Self) {
        self.red *= other.red;
        self.green *= other.green;
        self.blue *= other.blue;
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

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
pub type ColourMatrix = matrix::GenericMatrix<Spectrum>;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RGBSpectrum {
    pub red: Float,
    pub green: Float,
    pub blue: Float,
}

impl matrix::OneZero for RGBSpectrum {
    fn one()->Self{
        Self::gray(1.)
    }

    fn zero()->Self{
        Self::black()
    }
}

impl RGBSpectrum {
    pub fn black() -> Self {
        Self {
            red: 0.,
            green: 0.,
            blue: 0.,
        }
    }

    pub fn gray(v: Float) -> Self {
        Self {
            red: v,
            green: v,
            blue: v,
        }
    }

    pub fn is_black(&self)->bool{
        self.red == 0. && self.green == 0. && self.blue == 0.
    }
}

impl std::fmt::Display for RGBSpectrum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "RGB({:.5},{:.5},{:.5})",
            self.red, self.green, self.blue
        )
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

impl std::ops::DivAssign<Float> for RGBSpectrum {
    fn div_assign(&mut self, other: Float) {
        self.red /= other;
        self.green /= other;
        self.blue /= other;
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

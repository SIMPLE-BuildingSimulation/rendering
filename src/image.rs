
/*
 PART OF THIS FILE contains code to write four byte rgbe file format
 developed by Greg Ward. It handles the conversions between rgbe and
 pixels consisting of floats.

 This code was translated into Rust... The original work is available
 at http://www.graphics.cornell.edu/~bjw/
 written by Bruce Walter  (bjw@graphics.cornell.edu)  5/26/95
 based on code written by Greg Ward
*/
use crate::Float;
use crate::colour::Spectrum;
use std::io::Write;

fn rusty_frexp(s: Float) -> (Float, i32) {
    if 0.0 == s {
        return (s, 0);
    } else {
        let lg = s.abs().log2();
        let x = (lg - lg.floor() - 1.0).exp2();
        let exp = lg.floor() + 1.0;
        (s.signum() * x, exp as i32)
    }
}

fn float_to_rgbe(red: Float, green: Float, blue: Float) -> [u8; 4] {
    let mut v: Float;

    v = red;
    if green > v {
        v = green;
    }
    if blue > v {
        v = blue;
    }
    if v < Float::EPSILON {
        [0, 0, 0, 0]
    } else {
        let (mut mantissa, e) = rusty_frexp(v);
        mantissa *= 256.0 / v;
        let r = (red * mantissa).floor() as u8;
        let g = (green * mantissa).floor() as u8;
        let b = (blue * mantissa).floor() as u8;

        debug_assert!(e + 128 >= 0);
        debug_assert!(e + 128 <= u8::MAX as i32);

        let e = (e + 128) as u8;

        [r, g, b, e]
    }
}

/// A buffer with all the physical values in the image
/// (i.e., Radiance, Irradiance or whatever being calculated)
///
pub struct ImageBuffer {
    /// Number of columns
    pub width: usize,
    /// Number of rows
    pub height: usize,
    /// All the pixels, iterating from top
    /// to bottom, left to right
    pixels: Vec<Spectrum>,
}

impl std::ops::IndexMut<(usize, usize)> for ImageBuffer {
    fn index_mut(&mut self, pixel: (usize, usize)) -> &mut Self::Output {
        let (x, y) = pixel;
        let i = y * self.width + x;
        &mut self.pixels[i]
    }
}

impl std::ops::Index<(usize, usize)> for ImageBuffer {
    type Output = Spectrum;

    fn index(&self, pixel: (usize, usize)) -> &Self::Output {
        let (x, y) = pixel;
        let i = y * self.width + x;
        &self.pixels[i]
    }
}

impl ImageBuffer {
    /// Creates a new empty [`ImageBuffer`]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Spectrum::black(); width * height],
        }
    }

    /// Saves the image in HDRE format
    pub fn save_hdre(&self, filename: String) {
        // Create the file
        let mut file = std::fs::File::create(filename).unwrap();
        // Write header
        // let gamma = 1.0;
        // let exposure = 1.0;
        file.write_all(b"#?RGBE\n").unwrap();
        // file.write_all(format!("GAMMA={}\n", gamma).as_bytes()).unwrap();
        // file.write_all(format!("EXPOSURE={}\n", exposure).as_bytes()).unwrap();
        file.write_all(b"FORMAT=32-bit_rle_rgbe\n\n").unwrap();
        file.write(format!("-Y {} +X {}\n", self.height, self.width).as_bytes())
            .unwrap();

        for pixel in self.pixels.iter() {
            file.write(&float_to_rgbe(pixel.red, pixel.green, pixel.blue))
                .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PI;
    use super::*;
    use std::os::raw::{c_double, c_int};

    extern "C" {
        fn frexp(x: c_double, exp: *mut c_int) -> c_double;
    }

    fn c_frexp(x: Float) -> (Float, i32) {
        let mut exp: c_int = 0;
        let res = unsafe { frexp(x, &mut exp) };
        (res, exp as i32)
    }

    #[test]
    fn test_frexp() {
        let xs: Vec<Float> = vec![
            1e6,
            2.,
            PI,
            123987.,
            0.,
            99.,
            2.3123,
            1024.,
            0.1,
        ];
        for x in xs.iter() {
            let (c_mantissa, c_exp) = c_frexp(*x);
            let (mantissa, exp) = rusty_frexp(*x);
            println!(
                "... x={} | c_mant: {}, c_exp: {}; mant: {}, exp: {}",
                x, c_mantissa, c_exp, mantissa, exp
            );
            assert_eq!(exp, c_exp);
            assert!((mantissa - c_mantissa).abs() < 0.000000001);
        }
    }

    #[test]
    fn test_float_to_rgbe() {
        // Produced automatically
        assert_eq!(float_to_rgbe(807., 249., 73.), [201, 62, 18, 138]);
        assert_eq!(
            float_to_rgbe(984943658.000000, 1144108930.000000, 470211272.000000),
            [117, 136, 56, 159]
        );
        assert_eq!(
            float_to_rgbe(101027544.000000, 1457850878.000000, 1458777923.000000),
            [12, 173, 173, 159]
        );
        assert_eq!(
            float_to_rgbe(2007237709.000000, 823564440.000000, 1115438165.000000),
            [239, 98, 132, 159]
        );
        assert_eq!(
            float_to_rgbe(1784484492.000000, 74243042.000000, 114807987.000000),
            [212, 8, 13, 159]
        );
        assert_eq!(
            float_to_rgbe(1137522503.000000, 1441282327.000000, 16531729.000000),
            [135, 171, 1, 159]
        );
        assert_eq!(
            float_to_rgbe(823378840.000000, 143542612.000000, 896544303.000000),
            [196, 34, 213, 158]
        );
        assert_eq!(
            float_to_rgbe(1474833169.000000, 1264817709.000000, 1998097157.000000),
            [175, 150, 238, 159]
        );
        assert_eq!(
            float_to_rgbe(1817129560.000000, 1131570933.000000, 197493099.000000),
            [216, 134, 23, 159]
        );
        assert_eq!(
            float_to_rgbe(1404280278.000000, 893351816.000000, 1505795335.000000),
            [167, 106, 179, 159]
        );
    }
}

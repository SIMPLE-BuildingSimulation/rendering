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

/*
 PART OF THIS FILE contains code to write four byte rgbe file format
 developed by Greg Ward. It handles the conversions between rgbe and
 pixels consisting of floats.

 This code was translated into Rust... The original work is available
 at http://www.graphics.cornell.edu/~bjw/
 written by Bruce Walter  (bjw@graphics.cornell.edu)  5/26/95
 based on code written by Greg Ward
*/
use crate::colour::Spectrum;
use crate::Float;
use std::io::Write;
use std::path::Path;

/// Equivalent to C's `frexp` function
fn rusty_frexp(s: Float) -> (Float, i32) {
    if 0.0 == s {
        (s, 0)
    } else {
        let lg = s.abs().log2();
        let x = (lg - lg.floor() - 1.0).exp2();
        let exp = lg.floor() + 1.0;
        (s.signum() * x, exp as i32)
    }
}

/// Equivalent to C's `ldexp` function
fn rusty_ldexp(x: Float, n: i32) -> Float {
    x * (2. as Float).powi(n)
}

fn colour_to_rgbe(red: Float, green: Float, blue: Float) -> [u8; 4] {
    let mut v = red;
    if green > v {
        v = green;
    }
    if blue > v {
        v = blue;
    }
    if v < 1e-19 {
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

fn rgbe_to_colour(r: u8, g: u8, b: u8, e: u8) -> Spectrum {
    if e == 0 {
        return Spectrum::black();
    }

    let n = e as i32 - (128 + 8) as i32;
    let f = rusty_ldexp(1., n);
    let red = r as Float * f;
    let green = g as Float * f;
    let blue = b as Float * f;

    Spectrum { red, green, blue }
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
    pub pixels: Vec<Spectrum>,
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

    /// Creates a new empty [`ImageBuffer`]
    pub fn from_pixels(width: usize, height: usize, pixels: Vec<Spectrum>) -> Self {
        if pixels.len() != width * height {
            panic!("Width ({}) and Height ({}) does not match the number of pixels (n_pixels is {}... expecting width*height={})", width, height, pixels.len(), width*height)
        }
        Self {
            width,
            height,
            pixels,
        }
    }

    /// Saves the image in HDRE format
    pub fn save_hdre(&self, filename: &Path) {
        // Create the file
        let mut file = std::fs::File::create(filename).unwrap();
        // Write header
        // let gamma = 1.0;
        // let exposure = 1.0;
        file.write_all(b"#?RGBE\n").unwrap();
        // file.write_all(format!("GAMMA={}\n", gamma).as_bytes()).unwrap();
        // file.write_all(format!("EXPOSURE={}\n", exposure).as_bytes()).unwrap();
        file.write_all(b"FORMAT=32-bit_rle_rgbe\n\n").unwrap();
        file.write_all(format!("-Y {} +X {}\n", self.height, self.width).as_bytes())
            .unwrap();

        for pixel in self.pixels.iter() {
            file.write_all(&colour_to_rgbe(pixel.red, pixel.green, pixel.blue))
                .unwrap();
        }
    }

    /// Creates a new empty [`ImageBuffer`] from a File
    pub fn from_file(filename: &Path) -> Result<Self, String> {
        let content = match std::fs::read(filename) {
            Ok(v) => v,
            Err(_) => {
                return Err(format!(
                    "Could not read image file '{}'",
                    filename.to_str().unwrap()
                ))
            }
        };
        let filename = filename.to_str().unwrap();

        // Read header
        let mut height: Option<usize> = None;
        let mut width: Option<usize> = None;
        let mut lines = content.split(|c| (*c as char) == '\n');
        // READ HEADER
        // while let Some(line) = lines.next() {
        for line in lines.by_ref() {
            if line.starts_with(b"-Y") {
                let errmsg = {
                    let l = std::str::from_utf8(line).unwrap();
                    Err(format!("When reading file '{}' : Expecting SIZE line to be in the format '-Y number +X number'... found '{}'",filename, l))
                };

                // Size
                let tuple: Vec<&[u8]> = line
                    .split(|c| c.is_ascii_whitespace())
                    .into_iter()
                    .collect();
                if tuple.len() != 4 || tuple[2].ne(b"+X") {
                    return errmsg;
                }
                let l = std::str::from_utf8(tuple[1]).unwrap();
                height = match l.parse::<usize>() {
                    Ok(v) => Some(v),
                    Err(_) => {
                        return errmsg;
                    }
                };
                let l = std::str::from_utf8(tuple[3]).unwrap();
                width = match l.parse::<usize>() {
                    Ok(v) => Some(v),
                    Err(_) => {
                        return errmsg;
                    }
                };

                break; // Done with header
            }

            if line.starts_with(b"FORMAT") {
                // Format
                let tuple: Vec<&[u8]> = line.split(|c| *c == b'=').into_iter().collect();
                if tuple.len() != 2 {
                    let l = std::str::from_utf8(line).unwrap();
                    return Err(format!(
                        "Expecting FORMAT line to be in the format 'FORMAT=number'... found '{}'",
                        l
                    ));
                }
                let exp_format = b"32-bit_rle_rgbe";
                if tuple[1].ne(exp_format) {
                    let exp_format = std::str::from_utf8(exp_format).unwrap();
                    let found_format = std::str::from_utf8(tuple[1]).unwrap();
                    return Err(format!(
                        "Expecting FORMAT to be '{}'... found '{}'",
                        exp_format, found_format
                    ));
                };
                continue;
            }
        } // Done with header

        let width = width.unwrap();
        let height = height.unwrap();
        let mut pixels: Vec<Spectrum> = Vec::with_capacity(width * height);

        // Now the body
        let mut r: u8 = 0;
        let mut g: u8 = 0;
        let mut b: u8 = 0;
        let mut counter: u8 = 0; // Keep note on whether we are in r, g, b, or e
        for line in lines {
            // if !line.is_empty(){
            //     println!("Line length = {}", line.len());
            //     print!("Line --> ");
            //     for c in line.iter(){
            //         print!("{}|", *c);
            //     }
            //     println!("");
            // }
            // break;
            // for each line and
            // for each Byte in line
            // for i in 0..line.len() {
            for x in line {
                match counter {
                    0 => r = *x, //line[i],
                    1 => g = *x, //line[i],
                    2 => b = *x, //line[i],
                    3 => {
                        // When we register an e, we push value
                        let e = *x; //line[i];
                        pixels.push(rgbe_to_colour(r, g, b, e));
                    }
                    _ => unreachable!(),
                }
                counter += 1;
                counter %= 4;
            }
        } // Finished iterating lines

        // return
        Ok(Self {
            width,
            height,
            pixels,
        })
    } // end of from_file()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::PI;
    #[cfg(not(feature = "float"))]
    use std::os::raw::c_double;
    #[cfg(feature = "float")]
    use std::os::raw::c_float;
    use std::os::raw::c_int;

    #[cfg(not(feature = "float"))]
    extern "C" {
        fn frexp(x: c_double, exp: *mut c_int) -> c_double;
        fn ldexp(x: c_double, ex: c_int) -> c_double;
    }

    #[cfg(feature = "float")]
    extern "C" {
        fn frexp(x: c_float, exp: *mut c_int) -> c_float;
        fn ldexp(x: c_float, ex: c_int) -> c_float;
    }

    fn c_frexp(x: Float) -> (Float, i32) {
        let mut exp: c_int = 0;
        let res = unsafe { frexp(x, &mut exp) };
        (res, exp as i32)
    }

    fn c_ldexp(x: Float, n: i32) -> Float {
        unsafe { ldexp(x, n) }
    }

    #[test]
    fn test_frexp() {
        let xs: Vec<Float> = vec![1e6, 2., PI, 123987., 0., 99., 2.3123, 1024., 0.1];
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
    fn test_ldexp() {
        let is: Vec<i32> = vec![1, 2, 3, 4, 5, 6, -1, -2, -3, -4];
        let xs: Vec<Float> = vec![1e6, 2., PI, 123987., 0., 99., 2.3123, 1024., 0.1];
        for x in xs.iter() {
            for i in is.iter() {
                let c = c_ldexp(*x, *i);
                let r = rusty_ldexp(*x, *i);
                println!("{}*2^{} = {} in C and {} in Rust", x, i, c, r);
                assert_eq!(c, r);
            }
        }
    }

    #[test]
    fn test_colour_to_rgbe() {
        // Produced automatically
        assert_eq!(colour_to_rgbe(807., 249., 73.), [201, 62, 18, 138]);
        assert_eq!(
            colour_to_rgbe(984943658.000000, 1144108930.000000, 470211272.000000),
            [117, 136, 56, 159]
        );
        assert_eq!(
            colour_to_rgbe(101027544.000000, 1457850878.000000, 1458777923.000000),
            [12, 173, 173, 159]
        );
        assert_eq!(
            colour_to_rgbe(2007237709.000000, 823564440.000000, 1115438165.000000),
            [239, 98, 132, 159]
        );
        assert_eq!(
            colour_to_rgbe(1784484492.000000, 74243042.000000, 114807987.000000),
            [212, 8, 13, 159]
        );
        assert_eq!(
            colour_to_rgbe(1137522503.000000, 1441282327.000000, 16531729.000000),
            [135, 171, 1, 159]
        );
        assert_eq!(
            colour_to_rgbe(823378840.000000, 143542612.000000, 896544303.000000),
            [196, 34, 213, 158]
        );
        assert_eq!(
            colour_to_rgbe(1474833169.000000, 1264817709.000000, 1998097157.000000),
            [175, 150, 238, 159]
        );
        assert_eq!(
            colour_to_rgbe(1817129560.000000, 1131570933.000000, 197493099.000000),
            [216, 134, 23, 159]
        );
        assert_eq!(
            colour_to_rgbe(1404280278.000000, 893351816.000000, 1505795335.000000),
            [167, 106, 179, 159]
        );
    }

    #[test]
    #[ignore]
    fn test_rgbe_to_colour() {
        // Produced automatically
        assert_eq!(
            rgbe_to_colour(201, 62, 18, 138),
            Spectrum {
                red: 807.,
                green: 249.,
                blue: 73.
            }
        );
        assert_eq!(
            rgbe_to_colour(117, 136, 56, 159),
            Spectrum {
                red: 984943658.000000,
                green: 1144108930.,
                blue: 470211272.
            }
        );
        assert_eq!(
            rgbe_to_colour(12, 173, 173, 159),
            Spectrum {
                red: 101027544.000000,
                green: 1457850878.,
                blue: 1458777923.
            }
        );
        assert_eq!(
            rgbe_to_colour(239, 98, 132, 159),
            Spectrum {
                red: 2007237709.000000,
                green: 823564440.,
                blue: 1115438165.
            }
        );
        assert_eq!(
            rgbe_to_colour(212, 8, 13, 159),
            Spectrum {
                red: 1784484492.000000,
                green: 74243042.,
                blue: 114807987.
            }
        );
        assert_eq!(
            rgbe_to_colour(135, 171, 1, 159),
            Spectrum {
                red: 1137522503.000000,
                green: 1441282327.,
                blue: 16531729.
            }
        );
        assert_eq!(
            rgbe_to_colour(196, 34, 213, 158),
            Spectrum {
                red: 823378840.000000,
                green: 143542612.,
                blue: 896544303.
            }
        );
        assert_eq!(
            rgbe_to_colour(175, 150, 238, 159),
            Spectrum {
                red: 1474833169.000000,
                green: 1264817709.,
                blue: 1998097157.
            }
        );
        assert_eq!(
            rgbe_to_colour(216, 134, 23, 159),
            Spectrum {
                red: 1817129560.000000,
                green: 1131570933.,
                blue: 197493099.
            }
        );
        assert_eq!(
            rgbe_to_colour(167, 106, 179, 159),
            Spectrum {
                red: 1404280278.000000,
                green: 893351816.,
                blue: 1505795335.
            }
        );
    }

    #[test]
    #[ignore]
    fn test_from_file() {
        let buffer = ImageBuffer::from_file(Path::new("./test_data/images/cornell.hdr")).unwrap();
        assert_eq!(buffer.width, 1024);
        assert_eq!(buffer.height, 768);
        // assert_eq!(buffer.pixels.len(), 1024*768);
        buffer.save_hdre(Path::new("./test_data/images/cornell_COPIED.hdr"))
    }
}

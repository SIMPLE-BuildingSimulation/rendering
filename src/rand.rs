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

pub use rand::prelude::*;

pub type RandGen = SmallRng;

/// Gets a random number generator for Montecarlo estimations
pub fn get_rng()->RandGen{    
    // rand::thread_rng()
    SmallRng::from_entropy()
}

// THis function was copied from https://bheisler.github.io/post/writing-gpu-accelerated-path-tracer-part-2/
// But it does not seem to improve performance compared with
// the `rand::SmallRng`
// pub fn random_float(seed: &mut u32) -> f32 {
//     let mut x = *seed;
//     x ^= x >> 13;
//     x ^= x << 17;
//     x ^= x >> 5;
//     *seed = x;
//     let float_bits = (x & 0x007FFFFF) | 0x3F800000;
//     let float: f32 = unsafe { ::core::mem::transmute(float_bits) };
//     float - 1.0
// }

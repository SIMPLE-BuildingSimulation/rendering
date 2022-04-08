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
use crate::colour::Spectrum;

pub fn map_linear_colour(x: Float, min: Float, max: Float, map: &[Spectrum])->Spectrum{        
    if x <= min {
        return map[0]
    }else if x >= max {
        return *map.last().expect("Given an empty colour map")
    }
    
    let delta = (max - min)/(map.len() - 1) as Float ;
    for i in 1..map.len() {
        let bin_start = i as Float * delta;        
        let bin_end = bin_start + delta;
        if x <= bin_end {
            let lam = (x - bin_start)/delta;
            return map[i-1] + ( map[i] - map[i-1] )*lam ;
        }
    }
    unreachable!()
}
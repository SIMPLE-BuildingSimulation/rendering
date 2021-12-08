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

// Define whether we are working with
// doubles (i.e., f64) or floats (i.e., f32)

/// The kind of Floating point number used in the
/// library... the `"float"` feature means it becomes `f32`
/// and `f64` is used otherwise.
#[cfg(feature = "float")]
type Float = f32;
#[cfg(feature = "float")]
const PI: Float = std::f32::consts::PI;

#[cfg(not(feature = "float"))]
type Float = f64;
#[cfg(not(feature = "float"))]
const PI: Float = std::f64::consts::PI;

#[cfg(feature = "parallel")]
type RefCount<T> = std::sync::Arc<T>;
#[cfg(not(feature = "parallel"))]
type RefCount<T> = std::rc::Rc<T>;

// Core
pub mod ray;
pub mod interaction;
pub mod sampleable_trait;
pub mod samplers;
pub mod camera;
pub mod colour;
pub mod film;
pub mod from_radiance;
pub mod image;
pub mod material;
pub mod scene;
pub mod rand;
// pub mod lights;

// Ray-tracer
pub mod ray_tracer;

// Bidirectional Path Tracing modules
// pub mod bidirectional_path_tracer;
// pub mod subpath;
// pub mod vertex;

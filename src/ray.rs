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
use geometry3d::{Point3D, Ray3D, Transform, Vector3D};
use crate::interaction::Interaction;
/// Represents a ray (of light?) beyond pure geometry. It
/// includes also the current index of refraction and, potentially,
/// time (for blurry images)
#[derive(Clone)]
pub struct Ray {
    
    /// Direction and position
    pub geometry: Ray3D,
        
    /// index of refraction of the current medium
    pub refraction_index: Float,

    /// Contains the information about the last hit.
    pub interaction: Interaction,
}

impl Ray {
    pub fn transform(&mut self, t: &Transform) {
        let (geometry, _o_error, _d_error) = t.transform_ray(&self.geometry);        
        self.geometry = geometry;
            
    }

    #[inline(always)]
    pub fn direction(&self) -> Vector3D {
        self.geometry.direction
    }

    #[inline(always)]
    pub fn origin(&self) -> Point3D {
        self.geometry.origin
    }
}

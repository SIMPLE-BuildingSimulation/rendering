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
use geometry3d::{Transform, Ray3D, Point3D, Vector3D};

#[derive(Clone, Copy)]
pub struct Ray{
    pub geometry: Ray3D,
    pub time: Float,
    // medium.
}

impl Ray {

    pub fn apply_transformation(&self, t:&Transform)->Self{
        let (geometry,_o_error, _d_error)= t.transform_ray(&self.geometry);
        Self{
            geometry,
            time: self.time,
            // medium: self.medium
        }
    }

    #[inline(always)]
    pub fn direction(&self)->Vector3D{
        self.geometry.direction
    }

    #[inline(always)]
    pub fn origin(&self)->Point3D{
        self.geometry.origin
    }
}


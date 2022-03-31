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

use crate::ray::Ray;
use crate::Float;
use geometry3d::{Point3D, Vector3D};
use crate::material::Material;
use crate::colour::Spectrum;
use crate::rand::*;

/// A mirror material
pub struct Light(pub Spectrum);

impl Material for Light {
    fn id(&self) -> &str{
        "Light"
    }

    fn colour(&self) -> Spectrum{
        self.0
    }

    fn emits_direct_light(&self) -> bool{
        true
    }
    
    /// Should this material emits light    
    fn emits_light(&self) -> bool{
        true
    }

    fn sample_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        intersection_pt: Point3D,
        ray: Ray,
        rng: &mut RandGen,
    ) -> (Ray, Float, bool){
        panic!("{} material does not have a BSDF to sample", self.id())
    }

    fn eval_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        ray: &Ray,
        vout: Vector3D,
    ) -> Float{
        panic!("{} material does not have a BSDF to evaluate", self.id())
    }
}
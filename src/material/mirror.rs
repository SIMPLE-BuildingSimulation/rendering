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

use crate::material::specular::mirror_direction;
use crate::ray::Ray;
use crate::Float;
use geometry3d::{Point3D, Vector3D};
use crate::material::Material;
use crate::colour::Spectrum;
use crate::rand::*;

/// A mirror material
pub struct Mirror(pub Spectrum);

impl Material for Mirror {
    
    fn id(&self)->&str{
        "Mirror"
    }

    fn colour(&self)->Spectrum{
        self.0
    }

    fn specular_only(&self)->bool{
        true
    }

    fn get_possible_paths(
        &self,
        normal: &Vector3D,
        intersection_pt: &Point3D,
        ray: &Ray,
    ) -> [Option<(Ray, Float, Float)>; 2]{
            // Calculate the ray direction and BSDF
            let mut ray = ray.clone();
            let (v, _) = mirror_bsdf(*intersection_pt, &mut ray, *normal);
            [Some((ray, v, 1.)), None]
    }

    fn sample_bsdf(
        &self,
        normal: Vector3D,
        _e1: Vector3D,
        _e2: Vector3D,
        intersection_pt: Point3D,
        ray: &mut Ray,
        _rng: &mut RandGen,
    ) -> (Float, bool) {
        mirror_bsdf(intersection_pt, ray, normal)
    }
    
    fn eval_bsdf(
        &self,
        normal: Vector3D,
        _e1: Vector3D,
        _e2: Vector3D,
        ray: &Ray,
        vout: Vector3D,
    ) -> Float{
        let vin = ray.geometry.direction;
        eval_mirror_bsdf(normal, vin, vout)
    }

}



pub fn mirror_bsdf(intersection_pt: Point3D, ray: &mut Ray, normal: Vector3D) -> (Float, bool) {
    // avoid self shading
    // let mut ray = *ray;
    // let normal = *normal;

    ray.geometry.origin = intersection_pt + normal * 0.00001;
    let ray_dir = ray.geometry.direction;
    let cos = (ray_dir * normal).abs();
    ray.geometry.direction = mirror_direction(ray_dir, normal);
    debug_assert!((ray.geometry.direction.length() - 1.).abs() < 1e-8);
    ( 1. / cos, true)
    // (ray, 1., true)
}




pub fn eval_mirror_bsdf(normal: Vector3D, vin: Vector3D, vout: Vector3D) -> Float {
    let mirror = mirror_direction(vin, normal);
    if vout.is_parallel(mirror) {
        let cos = (vin * normal).abs();
        1. / cos
    } else {
        0.
    }
}

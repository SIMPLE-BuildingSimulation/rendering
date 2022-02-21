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

pub fn mirror_bsdf(intersection_pt: Point3D, mut ray: Ray, normal: Vector3D) -> (Ray, Float, bool) {
    // avoid self shading
    ray.geometry.origin = intersection_pt + normal * 0.00001;
    let ray_dir = ray.geometry.direction;
    let cos = (ray_dir * normal).abs();
    ray.geometry.direction = mirror_direction(ray_dir, normal);
    debug_assert!((ray.geometry.direction.length() - 1.).abs() < 1e-8);
    (ray, 1. / cos, true)
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

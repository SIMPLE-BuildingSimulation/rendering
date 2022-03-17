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

use crate::colour::Spectrum;
use crate::rand::*;
use crate::ray::Ray;
use crate::{Float, PI};
use geometry3d::{Point3D, Vector3D};

use crate::samplers::{local_to_world, sample_cosine_weighted_horizontal_hemisphere};

/// Information required for modelling Radiance's Plastic and Metal
pub struct PlasticMetal {
    pub color: Spectrum,
    pub specularity: Float,
    pub roughness: Float,
}



impl PlasticMetal {

    #[inline(always)]
    pub fn bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        intersection_pt: Point3D,
        mut ray: Ray,
        rng: &mut RandGen,
    ) -> (Ray, Float, bool) {
        // avoid self shading
        // let normal = *normal;
        // let mut ray = *ray;
        ray.geometry.origin = intersection_pt + normal * 0.00001;

        if self.specularity > 0. {
            unimplemented!()
        } else {
            // Probability
            const ONE_OVER_PI: Float = 1. / PI;
            let prob = ONE_OVER_PI;

            let local_dir = sample_cosine_weighted_horizontal_hemisphere(rng);
            debug_assert!(
                (local_dir.length() - 1.).abs() < 1e-6,
                "Length was {}",
                local_dir.length()
            );            
            let (x, y, z) = local_to_world(
                e1,
                e2,
                normal,
                Point3D::new(0., 0., 0.),
                local_dir.x,
                local_dir.y,
                local_dir.z,
            );
            let dir = Vector3D::new(x, y, z);            
            ray.geometry.direction = dir;
            debug_assert!( (dir.length() - 1.).abs() < 1e-4);
            (ray, prob, false)
        }
    }

    #[inline]
    pub fn eval_bsdf(
        &self,
        _normal: Vector3D,
        _e1: Vector3D,
        _e2: Vector3D,
        _vin: Vector3D,
        _vout: Vector3D,
    ) -> Float {
        if self.specularity > 0. {
            unimplemented!()
        } else {
            1. / PI
        }
    }
}

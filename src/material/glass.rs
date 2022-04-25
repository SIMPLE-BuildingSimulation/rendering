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
use crate::material::specular::*;
use crate::rand::*;
use crate::ray::Ray;
use crate::Float;
use geometry3d::{Point3D, Vector3D};

fn any_transmission(colour: &mut Spectrum) -> bool {
    const MIN_COLOUR: Float = 1e-15;
    if colour.max() < MIN_COLOUR {
        false
    } else {
        if colour.red < MIN_COLOUR {
            colour.red = MIN_COLOUR;
        }
        if colour.green < MIN_COLOUR {
            colour.green = MIN_COLOUR;
        }
        if colour.blue < MIN_COLOUR {
            colour.blue = MIN_COLOUR;
        }
        true
    }
}

pub struct Glass {
    pub colour: Spectrum,
    pub refraction_index: Float,
}

impl Glass {
    pub fn refl_trans(
        &self,
        n1: Float,
        cos1: Float,
        n2: Float,
        cos2: Option<Float>,
    ) -> (Float, Float) {
        debug_assert!(cos1 > 0.0);

        // Check if there is any transmission
        let mut colour = self.colour;
        let any_transmission = any_transmission(&mut colour);

        // Now calculate components
        if let Some(cos2) = cos2 {
            // There is refraction
            let ct = 1. / cos2;
            let ct2 = ct.powi(2);

            let fte = fresnel_te(n1, cos1, n2, cos2).powi(2);
            let fte2 = fte.powi(2);
            let ftm = fresnel_tm(n1, cos1, n2, cos2).powi(2);
            let ftm2 = ftm.powi(2);

            // Process transmission
            let t_comp = if any_transmission {
                0.5 * ct
                    * ((1.0 - fte).powi(2) / (1.0 - fte2 * ct2)
                        + (1.0 - ftm).powi(2) / (1.0 - ftm2 * ct2))
            } else {
                0.0
            };

            // Process reflection
            let refl_comp = 0.5
                * (fte * (1. + (1. - 2. * fte) * ct2) / (1. - fte2 * ct2)
                    + ftm * (1. + (1. - 2. * ftm) * ct2) / (1. - ftm2 * ct2));

            // return
            (refl_comp, t_comp)
        } else {
            // (0., 0.)
            // panic!("Glass should never reach critical angle");
            (1. / cos1, 0.)
        }
    }
}

impl Glass {
    pub fn id(&self) -> &str {
        "Glass"
    }

    pub fn colour(&self) -> Spectrum {
        let mut c = self.colour;
        _ = any_transmission(&mut c);
        c
    }

    pub fn get_possible_paths(
        &self,
        normal: &Vector3D,
        intersection_pt: &Point3D,
        ray: &Ray,
    ) -> [Option<(Ray, Float)>; 2] {
        let normal = *normal;
        // Only two possible direction:

        let mirror_dir = mirror_direction(ray.geometry.direction, normal);

        debug_assert!(
            // some paranoia checks
            (1. - mirror_dir.length()).abs() < 1e-5,
            "length is {}",
            mirror_dir.length()
        );
        let (n1, cos1, n2, cos2) = cos_and_n(ray, normal, self.refraction_index);
        let intersection_pt = *intersection_pt;
        let (refl, trans) = self.refl_trans(n1, cos1, n2, cos2);

        // Process reflection...
        let mut ray1 = *ray;
        ray1.geometry.direction = mirror_dir;
        ray1.geometry.origin = intersection_pt + normal * 0.00001;
        let pair1 = Some((ray1, refl));

        // process transmission
        let mut ray = *ray;
        let pair2 = if trans > 0.0 {
            ray.geometry.origin = intersection_pt - normal * 0.00001;
            ray.colour *= self.colour() * trans;
            Some((ray, trans))
        } else {
            None
        };

        [pair1, pair2]
    }

    pub fn sample_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        intersection_pt: Point3D,
        ray: &mut Ray,
        rng: &mut RandGen,
    ) -> (Spectrum, Float) {
        debug_assert!(
            (ray.geometry.direction.length() - 1.).abs() < 1e-5,
            "Length was {}",
            ray.geometry.direction.length()
        );
        debug_assert!((e1 * e2).abs() < 1e-8);
        debug_assert!((e1 * normal).abs() < 1e-8);
        debug_assert!((e2 * normal).abs() < 1e-8);

        let (n1, cos1, n2, cos2) = cos_and_n(ray, normal, self.refraction_index);
        let (refl, trans) = self.refl_trans(n1, cos1, n2, cos2);
        let ray_dir = ray.geometry.direction;
        let mirror_dir = mirror_direction(ray_dir, normal);
        debug_assert!(
            (1. - mirror_dir.length()).abs() < 1e-5,
            "length is {}",
            mirror_dir.length()
        );

        let r: Float = rng.gen();
        if r <= refl / (refl + trans) {
            // Reflection
            // avoid self shading
            ray.geometry.origin = intersection_pt + normal * 0.00001;

            ray.geometry.direction = mirror_dir;
            (Spectrum::gray(1.) * refl, refl / (refl + trans))
        } else {
            // Transmission... keep same direction, dont change refraction
            // avoid self shading
            ray.geometry.origin = intersection_pt - normal * 0.00001;
            (self.colour * trans, trans / (refl + trans))
        }
    }

    pub fn eval_bsdf(
        &self,
        normal: Vector3D,
        _e1: Vector3D,
        _e2: Vector3D,
        ray: &Ray,
        vout: Vector3D,
    ) -> Spectrum {
        let (n1, cos1, n2, cos2) = cos_and_n(ray, normal, self.refraction_index);
        let (refl, trans) = self.refl_trans(n1, cos1, n2, cos2);
        let vin = ray.geometry.direction;
        let mirror_dir = mirror_direction(vin, normal);
        debug_assert!(
            (1. - mirror_dir.length()).abs() < 1e-5,
            "length is {}",
            mirror_dir.length()
        );

        // If reflection
        if vout.is_same_direction(mirror_dir) {
            return Spectrum::gray(refl);
        }

        let mut colour = self.colour;
        if any_transmission(&mut colour) {
            // it is not refraction either
            return Spectrum::black();
        }
        // Check transmission
        if let Some(_cos2) = cos2 {
            if vout.is_same_direction(vin) {
                return self.colour * trans;
            }
        }
        // panic!("Glass should never reach critical angle");
        Spectrum::gray(1.) / cos1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geometry3d::{Ray3D, Vector3D};

    #[test]
    fn test_get_possible_paths_glass() {
        let glass = Glass {
            colour: Spectrum {
                red: 0.1,
                green: 0.2,
                blue: 0.3,
            },
            refraction_index: 1.52,
        };
        let mut rng = get_rng();

        for _ in 0..500 {
            let refraction_index: Float = rng.gen();
            let (x, y, z): (Float, Float, Float) = rng.gen();
            let direction = Vector3D::new(x, y, -z).get_normalized();

            let normal = Vector3D::new(0., 0., 1.);
            let intersection_pt = Point3D::new(0., 0., 0.);
            let ray = Ray {
                geometry: Ray3D {
                    origin: Point3D::new(0., 0., 2.),
                    direction,
                },
                refraction_index,
                ..Ray::default()
            };

            let paths = glass.get_possible_paths(&normal, &intersection_pt, &ray);
            // Reflection
            if let Some((new_ray, bsdf)) = paths[0] {
                assert_eq!(
                    new_ray.refraction_index, refraction_index,
                    "Expecting the ray's refraction index to be {}... found {}",
                    refraction_index, ray.refraction_index
                );
                assert!(
                    bsdf.is_finite() && !bsdf.is_nan(),
                    "impossible BSDF --> {}",
                    bsdf
                );
                let new_dir = new_ray.geometry.direction;
                assert!(( (new_dir.x - direction.x).abs() < 1e-5 && (new_dir.y - direction.y).abs() < 1e-5 && (new_dir.z  + direction.z).abs() < 1e-5 ), "Expecting reflected direction to be mirrored against direction (ray.dir = {} | exp = {}).", ray.geometry.direction, direction);
            } else {
                panic!("Expecting a reflection path")
            }

            // Transmission
            if let Some((new_ray, bsdf)) = paths[1] {
                assert_eq!(
                    new_ray.refraction_index, refraction_index,
                    "Expecting the ray's refraction index to be {}... found {}",
                    refraction_index, ray.refraction_index
                );
                assert!(
                    bsdf.is_finite() && !bsdf.is_nan(),
                    "impossible BSDF --> {}",
                    bsdf
                );
                assert!(new_ray.geometry.direction.compare(direction), "Expecting transmitted direction to be the same as the original direction (ray.dir = {} | exp = {})... length of diff = {}", ray.geometry.direction, direction, (new_ray.geometry.direction - direction).length());
            }
        }
    }
}

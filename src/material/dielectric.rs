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
use crate::material::Material;

pub struct Dielectric {
    pub colour: Spectrum,
    pub refraction_index: Float,
}

impl Dielectric {
    
    /// Gets the Reflected and Transmitted BSDF values
    pub fn refl_trans(
        &self,
        n1: Float,
        cos1: Float,
        n2: Float,
        cos2: Option<Float>,
    ) -> (Float, Float) {
        debug_assert!(cos1 > 0.0);
        if let Some(cos2) = cos2 {
            // There is refraction
            let refl = fresnel_reflectance(n1, cos1, n2, cos2);
            let refl_comp = refl / cos1;
            // This is one source of non-symmetrical BSDF
            // (check Eric Veach's thesis, chapter 5 )
            /* IF RADIANCE */
            let ratio = n2 / n1;
            let n_ratio2 = ratio * ratio;
            let t_comp = (1. - refl) * n_ratio2 / cos1;
            /* IF IMPORTANCE */
            // let t_comp = (1. - refl) / cos1;

            // return
            (refl_comp, t_comp)
        } else {
            // pure reflection
            // (1. / cos1, 0.)
            (1. / cos1, 0.)
        }
    }

    
    

    
}

impl Material for Dielectric {

    fn id(&self)->&str{
        "Dielectric"
    }

    fn colour(&self)->Spectrum{
        self.colour
    }
     

    fn specular_only(&self) -> bool{
        true
    }

    fn get_possible_paths(
        &self,
        normal: &Vector3D,
        intersection_pt: &Point3D,
        ray: &Ray,
    ) -> [Option<(Ray, Float, Float)>; 2]{
        let normal = *normal;        
        let intersection_pt = *intersection_pt;

        let (n1, cos1, n2, cos2) = cos_and_n(ray, normal, self.refraction_index);
        let (refl, trans) = self.refl_trans(n1, cos1, n2, cos2);
        let ray_dir = ray.geometry.direction;
        let mirror_dir = mirror_direction(ray_dir, normal);
        debug_assert!(
            (1. - mirror_dir.length()).abs() < 1e-5,
            "length is {}",
            mirror_dir.length()
        );

        // Process reflection...
        let mut ray1 = ray.clone();
        ray1.geometry.direction = mirror_dir;
        ray1.geometry.origin = intersection_pt + normal * 0.00001;
        let pair1 = Some((ray1, refl, cos1 * refl /*refl / (refl + trans)*/));

        let mut ray = ray.clone();
        // process transmission
        let pair2 = if trans > 0.0 {
            ray.geometry.origin = intersection_pt - normal * 0.00001;
            ray.refraction_index = n2;
            let trans_dir =
                fresnel_transmission_dir(ray_dir, normal, n1, cos1, n2, cos2.unwrap());
            ray.geometry.direction = trans_dir;
            Some((ray, trans, /*trans / (refl + trans)*/ 1. - cos1 * refl))
        } else {
            None
        };

        [pair1, pair2]
    }

    fn sample_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        intersection_pt: Point3D,
        ray: &mut Ray,
        rng: &mut RandGen,
    ) -> (Float, bool) {
        debug_assert!(
            (ray.geometry.direction.length() - 1.).abs() < 1e-5,
            "Length was {}",
            ray.geometry.direction.length()
        );
        debug_assert!((e1 * e2).abs() < 1e-5, "e1*e2= {} ", (e1 * e2).abs());
        debug_assert!((e1 * normal).abs() < 1e-5, "e1*normal = {}", e1*normal.abs());
        
        debug_assert!((e2 * normal).abs() < 1e-5, "e2*normal = {}",(e2 * normal).abs());

        let (n1, cos1, n2, cos2) = cos_and_n(ray, normal, self.refraction_index);
        let (refl, trans) = self.refl_trans(n1, cos1, n2, cos2);        
        let mirror_dir = mirror_direction(ray.geometry.direction, normal);
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
            (refl, true)
        } else {
            // Transmission
            // avoid self shading
            ray.geometry.origin = intersection_pt - normal * 0.00001;

            ray.refraction_index = n2;
            let trans_dir = fresnel_transmission_dir(ray.geometry.direction, normal, n1, cos1, n2, cos2.unwrap());
            ray.geometry.direction = trans_dir;
            (trans, true)
        }
    }

    fn eval_bsdf(
        &self,
        normal: Vector3D,
        _e1: Vector3D,
        _e2: Vector3D,
        ray: &Ray,
        vout: Vector3D,
    ) -> Float{
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
            return refl;
        }

        // Check transmission
        if let Some(cos2) = cos2 {
            // There is Reflection and transmission
            let trans_dir = fresnel_transmission_dir(vin, normal, n1, cos1, n2, cos2);
            debug_assert!(
                (1. - trans_dir.length()).abs() < 1e-5,
                "length is {}",
                trans_dir.length()
            );
            if vout.is_same_direction(trans_dir) {
                // transmission
                return trans;
            }
        }

        // Neither...
        0.0
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    use crate::{ray::Ray, interaction::Interaction};
    use geometry3d::{Point3D, Ray3D};
    #[test]
    fn test_normal_incidence() {
        // Example found online, a glass_air transition
        let n1 = 1.52; // glass
        let n2 = 1.; // air

        let normal = Vector3D::new(0., 0., 1.);

        let mat = Dielectric {
            colour: Spectrum::gray(0.1), //irrelevant for this test
            refraction_index: n2,
        };

        // Perpendicular rays aren't deviated
        let ray = Ray {
            refraction_index: n1,
            geometry: Ray3D {
                origin: Point3D::new(0., 0., 10.),
                direction: Vector3D::new(0., 0., -1.),
            },
            interaction: Interaction::default()
        };

        let (np1, cos1, np2, cos2) = cos_and_n(&ray, normal, mat.refraction_index);
        assert!((n1 - np1).abs() < 1e-8, "np1 = {}, n1 = {}", np1, n1);
        assert!((n2 - np2).abs() < 1e-8, "np2 = {}, n2 = {}", np2, n2);
        assert!((1. - cos1).abs() < 1e-8, "cos1 = {}", cos1);
        assert!(cos2.is_some());
        let cos2 = cos2.unwrap();
        assert!((1. - cos2).abs() < 1e-8, "cos2 = {}", cos2);
    }

    #[test]
    fn test_critical_angle() {
        // Example found online, a glass_air transition
        let n1 = 1.52 as Float; // glass
        let n2 = 1.003 as Float; // air

        let normal = Vector3D::new(0., 0., 1.);

        let mat = Dielectric {
            colour: Spectrum::gray(0.1), //irrelevant for this test
            refraction_index: n2,
        };

        let crit = (n2 / n1).asin();

        let direction = |angle: Float| -> Vector3D {
            let direction = Vector3D::new(0., angle.sin(), -angle.cos());
            let found_angle = (direction * normal).abs().acos();
            assert!(
                (found_angle - angle).abs() < 1e-4,
                "angle = {} | found_angle = {}",
                angle,
                found_angle
            );
            direction
        };

        // Check before critical angle
        let mut angle = 0.;
        let angle_d = 0.1;
        while angle < crit {
            let ray = Ray {
                refraction_index: n1,
                geometry: Ray3D {
                    origin: Point3D::new(0., 0., 10.),
                    direction: direction(angle.to_radians()),                    
                },
                interaction: Interaction::default()
            };

            let (_np1, _cos1, _np2, cos2) = cos_and_n(&ray, normal, mat.refraction_index);
            assert!(cos2.is_some());
            angle += angle_d;
        }

        // Check critical angle
        angle = crit;
        let ray = Ray {
            refraction_index: n1,
            geometry: Ray3D {
                origin: Point3D::new(0., 0., 10.),
                direction: direction(angle.to_radians()),
            },
            interaction: Interaction::default()
        };

        let (_np1, _cos1, _np2, cos2) = cos_and_n(&ray, normal, mat.refraction_index);
        assert!(cos2.is_some());
        angle += angle_d;

        // Check beyond critical angle
        while angle < crit {
            let ray = Ray {
                refraction_index: n1,
                geometry: Ray3D {
                    origin: Point3D::new(0., 0., 10.),
                    direction: direction(angle.to_radians()),
                },
                interaction: Interaction::default()
            };

            let (_np1, _cos1, _np2, cos2) = cos_and_n(&ray, normal, mat.refraction_index);
            assert!(cos2.is_some());
            angle += angle_d;
        }
    }

    #[test]
    fn test_sin_cos_n() {
        let n = 1.52;
        let mat = Dielectric {
            colour: Spectrum::gray(0.23), //irrelevant for this test
            refraction_index: n,
        };

        let normal = Vector3D::new(0., 0., 1.);

        let dir_zero = Vector3D::new(0., 1., -2.).get_normalized(); // going down

        let ray = Ray {
            geometry: Ray3D {
                origin: Point3D::new(0., 0., 0.),
                direction: dir_zero,
            },
            interaction: Interaction::default(),
            refraction_index: 2.9,
        };

        let (n1, cos1, n2, cos2) = cos_and_n(&ray, normal, mat.refraction_index);
        let theta1 = cos1.acos();
        let theta2 = cos2.unwrap().acos();
        let fresnel_1 = n1 * theta1.sin();
        let fresnel_2 = n2 * theta2.sin();

        assert!(
            (fresnel_1 - fresnel_2).abs() < 1e-5,
            "n1*sin1 = {} | n2*sin2 = {}",
            fresnel_1,
            fresnel_2
        );
    }

    #[test]
    fn test_bsdf() {
        let n = 1.52;
        let mat = Dielectric {
            colour: Spectrum::gray(0.23), //irrelevant for this test
            refraction_index: n,
        };

        let mut rng = get_rng();
        let normal = Vector3D::new(0., 0., 1.);
        let e1 = Vector3D::new(1., 0., 0.);
        let e2 = Vector3D::new(0., 1., 0.);

        let dir_zero = Vector3D::new(0., 1., -2.).get_normalized(); // going down

        let mut ray = Ray {
            geometry: Ray3D {
                origin: Point3D::new(0., 0., 0.),
                direction: dir_zero,
            },
            refraction_index: 1.,
            interaction: Interaction::default()
        };
        println!("Before entering: {}", dir_zero);
        let mirror_dir = mirror_direction(ray.geometry.direction, normal);
        let mut trans_dir: Option<Vector3D> = None;
        // Get INTO the material
        for _ in 0..30 {
            let mut new_ray = ray.clone();
            let ( _pdf, _is_specular) =
                mat.sample_bsdf(normal, e1, e2, Point3D::new(0., 0., 0.), &mut new_ray, &mut rng);
            println!("A -- PDF = {}", _pdf);

            let new_dir = new_ray.geometry.direction;
            if new_dir.z < 0. {
                // We are still moving down... thus, refraction
                assert!(
                    new_ray.refraction_index == n,
                    "Expeting n={}, found n={}",
                    n,
                    new_ray.refraction_index
                );
                trans_dir = Some(new_dir);

            } else {
                // reflection
                assert!(new_dir.is_same_direction(mirror_dir));
                assert!(
                    new_ray.refraction_index == 1.0,
                    "Expeting n={}, found n={}",
                    1.,
                    new_ray.refraction_index
                );
            }
        }

        println!("Inside: {:?}", trans_dir);

        // Get OUT OF the material
        ray.refraction_index = n;
        ray.geometry.direction = trans_dir.unwrap();
        for _ in 0..30 {
            let mut new_ray = ray.clone();
            let (_pdf, _is_specular) =
                mat.sample_bsdf(normal, e1, e2, Point3D::new(0., 0., 0.), &mut new_ray, &mut rng);
            println!("B -- PDF = {}", _pdf);
            let new_dir = new_ray.geometry.direction;
            if new_dir.z < 0. {
                // We are still moving down... thus, refraction
                assert!(
                    new_ray.refraction_index == 1.,
                    "Expeting n={}, found n={}",
                    1,
                    new_ray.refraction_index
                );
                assert!(
                    new_dir.is_same_direction(dir_zero),
                    "ray_dir = {} | new_dir = {} | dir_zero = {}",
                    new_ray.geometry.direction,
                    new_dir,
                    dir_zero
                );
                println!("After leaving : {}", new_dir);
            }
        }
    }

}

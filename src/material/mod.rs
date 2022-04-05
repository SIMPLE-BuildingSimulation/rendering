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
use crate::Float;
use geometry3d::{Point3D, Vector3D};


mod light;
pub use light::Light;

mod plastic;
pub use plastic::Plastic;

mod metal;
pub use metal::Metal;

mod dielectric;
pub use dielectric::Dielectric;

mod mirror;
pub use mirror::Mirror;

mod glass;
pub use glass::Glass;

mod specular;
pub use specular::*;

// pub enum Material {
//     Plastic(PlasticMetal),
//     Metal(PlasticMetal),
//     Light(Spectrum),
//     Mirror(Spectrum),
//     Dielectric(Dielectric),
// }

pub trait Material {

    /// Returns an id, for debugging and error reporting purposes
    fn id(&self) -> &str;


    /// Retrieves the Colour of the material. This will usually
    /// represent the values that will multiply the different
    /// elements of the [`Spectrum`]. E.g., the reflectance values.
    fn colour(&self) -> Spectrum;


    /// Should this material be tested for direct illumination?    
    fn emits_direct_light(&self) -> bool{
        false
    }
    

    /// Should this material emits light    
    fn emits_light(&self) -> bool{
        false
    }
    
    /// Does this material scatter (e.g., like [`Plastic`]) or does it
    /// only transmit/reflects specularly (e.g., like [`Mirror`])?
    ///
    /// Defaults to `false`
    fn specular_only(&self) -> bool{
        false
    }
    
    fn get_possible_paths(
        &self,
        _normal: &Vector3D,
        _intersection_pt: &Point3D,
        _ray: &Ray,
    ) -> [Option<(Ray, Float, Float)>; 2]{
        panic!("Calling unimplemented method get_possible_paths() for material '{}'", self.id())
    }
    

    /// Samples the bsdf (returned by modifying the given `Ray`).
    /// Returns the value of the BSDF in that direction, and a boolean
    /// indicating whether this is a specular or a diffuse interaction    
    fn sample_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        intersection_pt: Point3D,
        ray: &mut Ray,
        rng: &mut RandGen,
    ) -> (Float, bool) ;
    

    /// Evaluates a BSDF based on an input and outpt directions
    fn eval_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        ray: &Ray,
        vout: Vector3D,
    ) -> Float; 
    
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_vectors(rng: &mut RandGen) -> (Vector3D, Vector3D, Vector3D, Ray, Vector3D) {
        let e1 = Vector3D::new(rng.gen(), rng.gen(), rng.gen()).get_normalized();
        let e2 = e1.get_perpendicular().unwrap();
        let normal = e1.cross(e2);
        let direction = geometry3d::Vector3D::new(rng.gen(), rng.gen(), rng.gen()).get_normalized();

        // We need the direction to be opposite to normal.
        if direction * normal < 0. {
            let ray = Ray {
                geometry: geometry3d::Ray3D {
                    direction,
                    origin: geometry3d::Point3D::new(rng.gen(), rng.gen(), rng.gen()),
                },
                refraction_index: rng.gen(),
                .. Ray::default()
            };
            let vout = Vector3D::new(1., 4., 12.).get_normalized();

            (normal, e1, e2, ray, vout)
        } else {
            get_vectors(rng)
        }
    }

    fn test_material(material: Box<dyn Material>) {
        let mut rng = crate::rand::get_rng();
        for _ in 0..99999 {
            let (normal, e1, e2, mut ray, vout) = get_vectors(&mut rng);
            let old_ray = ray.clone();
            let (pdf, _is_specular) = material.sample_bsdf(normal, e1, e2, Point3D::new(0., 0., 0.), &mut ray, &mut rng);
            assert!(pdf.is_finite());
            assert!(old_ray.geometry.direction.length().is_finite());
            assert!(old_ray.geometry.origin.as_vector3d().length().is_finite());
            let pdf = material.eval_bsdf(normal, e1, e2, &old_ray, vout);
            assert!(pdf.is_finite());
        }
    }

    #[test]
    fn test_sample_plastic() {
        let plastic = Box::new(Plastic {
            colour: Spectrum {
                red: 0.5,
                green: 0.2,
                blue: 0.9,
            },
            specularity: 0.0,
            roughness: 0.0,
        });

        
        println!("{}", std::mem::size_of_val(&plastic));
        test_material(plastic)
    }

    #[test]
    fn test_sample_metal() {
        let metal = Box::new(Metal {
            colour: Spectrum {
                red: 0.5,
                green: 0.2,
                blue: 0.9,
            },
            specularity: 0.0,
            roughness: 0.0,
        });

        test_material(metal)
    }

    #[test]
    fn test_sample_mirror() {
        let mirror = Box::new(Mirror(Spectrum {
            red: 0.5,
            green: 0.2,
            blue: 0.9,
        }));
        test_material(mirror)
    }

    #[test]
    fn test_sample_dielectric() {
        let dielectric = Box::new(Dielectric {
            colour: Spectrum {
                red: 0.5,
                green: 0.2,
                blue: 0.9,
            },
            refraction_index: 1.,
        });
        test_material(dielectric)
    }
}

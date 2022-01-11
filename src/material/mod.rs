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
use crate::{Float};
use crate::colour::Spectrum;
use geometry3d::{Vector3D, Point3D};
use crate::rand::*;

mod plastic_metal;
pub use plastic_metal::PlasticMetal;

mod dielectric;
pub use dielectric::Dielectric;

mod mirror;
pub use mirror::*;

mod specular;
pub use specular::*;

pub enum Material {
    Plastic(PlasticMetal),
    Metal(PlasticMetal),
    Light(Spectrum),
    Mirror(Spectrum),
    Dielectric(Dielectric)
}

impl Material {
    /// Retrieves the Colour of the material. This will usually
    /// represent the values that will multiply the different
    /// elements of the [`Spectrum`]. E.g., the reflectance values.
    pub fn colour(&self) -> Spectrum{
        match self{
            Self::Plastic(s)=>s.color,
            Self::Metal(s)=>s.color,
            Self::Light(s)=>*s,
            Self::Mirror(s)=>*s,
            Self::Dielectric(s)=>s.color,
        }
    }

    /// Should this material be tested for direct illumination?    
    pub fn emits_direct_light(&self) -> bool {
        match self{
            Self::Light(_)=>true,
            _ => false
        }
    }

    /// Should this material emits light    
    pub fn emits_light(&self) -> bool {
        match self{
            Self::Light(_)=>true,
            _ => false
        }
    }

    
    /// Does this material scatter (e.g., like [`Plastic`]) or does it
    /// only transmit/reflects specularly (e.g., like [`Mirror`])?
    ///
    /// Defaults to `false`
    pub fn specular_only(&self) -> bool {
        match self{            
            Self::Mirror(_)=>true,
            Self::Dielectric(_)=>true,
            _ => false
        }
    }

    /// Samples the bsdf, returns a new direction, the value of the BSDF, and a boolean
    /// indicating whether this is a specular or a diffuse interaction
    pub fn sample_bsdf(&self, normal: Vector3D, e1: Vector3D, e2: Vector3D, intersection_pt: Point3D, ray: Ray, rng: &mut RandGen)->(Ray,Float, bool){

        debug_assert!( (ray.geometry.direction.length() - 1.).abs() < 1e-5, "Length was {}", ray.geometry.direction.length());
        debug_assert!( (e1*e2).abs() < 1e-8);
        debug_assert!( (e1*normal).abs() < 1e-8);
        debug_assert!( (e2*normal).abs() < 1e-8);
    
        match self {
            Self::Plastic(s)=>s.bsdf(normal, e1, e2, intersection_pt, ray, rng),
            Self::Metal(s)=>s.bsdf(normal, e1, e2, intersection_pt, ray, rng),
            Self::Light(_)=>panic!("Trying to build a BSDF for a Light material"),
            Self::Mirror(_)=>mirror_bsdf(intersection_pt, ray, normal),
            Self::Dielectric(s)=>s.bsdf(normal, intersection_pt, ray, rng),
        }
    }

    pub fn eval_bsdf(&self, normal: Vector3D, e1: Vector3D, e2: Vector3D, ray: Ray, vout: Vector3D)->Float{
        let vin = ray.geometry.direction;
        debug_assert!( (vin.length() - 1.).abs() < 1e-5, "Length was {}", vin.length());
        debug_assert!( (e1*e2).abs() < 1e-5);
        debug_assert!( (e1*normal).abs() < 1e-5);
        debug_assert!( (e2*normal).abs() < 1e-5);
        match self{
            Self::Plastic(s)=>s.eval_bsdf(normal, e1, e2, vin, vout),
            Self::Metal(s)=>s.eval_bsdf(normal, e1, e2, vin, vout),
            Self::Light(_)=>panic!("Trying to evaluate a BSDF for a Light material"),
            Self::Mirror(_)=>eval_mirror_bsdf(normal, vin, vout),
            Self::Dielectric(s)=>s.eval_bsdf(normal, ray, vout),
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    

    fn get_vectors(rng: &mut RandGen) -> (Vector3D,Vector3D,Vector3D,Ray,Vector3D) {        
        let e1 = Vector3D::new(rng.gen(), rng.gen(), rng.gen()).get_normalized();
        let e2 = e1.get_perpendicular().unwrap();
        let normal = e1.cross(e2);
        let direction = geometry3d::Vector3D::new(rng.gen(), rng.gen(), rng.gen()).get_normalized();
        
        // We need the direction to be opposite to normal.
        if direction*normal < 0. {
            let ray = Ray{
                geometry: geometry3d::Ray3D{
                    direction,
                    origin: geometry3d::Point3D::new(rng.gen(), rng.gen(), rng.gen())
                },
                refraction_index: rng.gen(),
            };
            let vout = Vector3D::new(1., 4., 12.).get_normalized();
    
            (normal, e1, e2, ray, vout)
        }else{
            get_vectors(rng)
        }
        
    }

    fn test_material(material: Material){

        let mut rng = crate::rand::get_rng();
        for _ in 0..999999{
            let (normal, e1, e2, ray, vout) = get_vectors(&mut rng);
            let (new_ray, pdf, _is_specular ) = material.sample_bsdf(normal, e1, e2, Point3D::new(0., 0., 0.), ray, &mut rng);
            assert!(pdf.is_finite());
            assert!(new_ray.geometry.direction.length().is_finite());
            assert!(new_ray.geometry.origin.as_vector3d().length().is_finite());
            let pdf = material.eval_bsdf(normal, e1,e2,ray,vout);
            assert!(pdf.is_finite());
        }
    }

    #[test]
    fn test_sample_plastic(){
        
        
        
        let plastic = Material::Plastic(PlasticMetal{
            color: Spectrum{red: 0.5, green: 0.2, blue: 0.9},
            specularity: 0.0, 
            roughness: 0.0
        });

        test_material(plastic)

    }

    #[test]
    fn test_sample_metal(){
        
        
        let metal = Material::Metal(PlasticMetal{
            color: Spectrum{red: 0.5, green: 0.2, blue: 0.9},
            specularity: 0.0, 
            roughness: 0.0
        });

        test_material(metal)

    }

    #[test]
    fn test_sample_mirror(){                
        let mirror = Material::Mirror(Spectrum{red: 0.5, green: 0.2, blue: 0.9});
        test_material(mirror)

    }

    #[test]
    fn test_sample_dielectric(){                
        let dielectric = Material::Dielectric(Dielectric{
            color: Spectrum{red: 0.5, green: 0.2, blue: 0.9},
            refraction_index: 1.,
        });
        test_material(dielectric)

    }
}

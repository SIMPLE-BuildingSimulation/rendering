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
use crate::{Float,PI};
use crate::colour::Spectrum;
use crate::samplers::{sample_cosine_weighted_horizontal_hemisphere, local_to_world};
use geometry3d::{Vector3D, Point3D};
use crate::rand::RandGen;
use rand::prelude::*;

/// Fresnel Coefficient for TE-Polarized Light, according to Radiance's 
/// material documentation
/// 
/// `n1` is the index of refraction on the ray's side; `cos1` is the 
/// cosine of the angle between the surface's normal and ray.
/// 
/// `n2` is the index of refraction on the side opposite to the ray; `cos2` is the 
/// cosine of the angle between the surface's normal and transmitted ray
fn fresnel_te(n1:Float, cos1:Float, n2: Float, cos2:Float )->Float{
    let ret = (n1*cos1-n2*cos2)/(n1*cos1+n2*cos2);
    ret*ret
}

/// Fresnel Coefficient for TM-Polarized Light, according to Radiance's 
/// material documentation
/// 
/// `n1` is the index of refraction on the ray's side; `cos1` is the 
/// cosine of the angle between the surface's normal and ray.
/// 
/// `n2` is the index of refraction on the side opposite to the ray; `cos2` is the 
/// cosine of the angle between the surface's normal and transmitted ray
fn fresnel_tm(n1:Float, cos1:Float, n2: Float, cos2:Float )->Float{
    let one_cos1 = 1./cos1;
    let one_cos2 = 1./cos2;
    fresnel_te(n1, one_cos1, n2, one_cos2)
}


fn fresnel_reflectance(n1:Float, cos1:Float, n2: Float, cos2:Float )->Float{
    0.5*(fresnel_tm(n1, cos1, n2, cos2) + fresnel_te(n1, cos1, n2, cos2))
}


/// Calculates the direction of the transmision
fn fresnel_transmission_dir(vin:Vector3D, normal: Vector3D, n1:Float, cos1:Float, n2: Float, cos2:Float )->Vector3D{
    let n_ratio = n1/n2;
    vin*n_ratio + normal*(n_ratio * cos1 + cos2)
}

/// Calculates the purely specular reflection direction.
fn mirror_direction(vin: Vector3D, normal: Vector3D) -> Vector3D {
    debug_assert!((vin.length() - 1.).abs() < 1e-6);
    debug_assert!((normal.length() - 1.).abs() < 1e-6);
    let mut ret = vin - normal * (2. * (vin * normal));
    ret.normalize();
    ret
}


/*************** */
/* DIELECTRIC    */
/*************** */

pub struct Dielectric {
    pub color: Spectrum,
    pub refraction_index: Float,
}

impl Dielectric {

    /// Calculates the parameters necessary for calculating the 
    /// Fresnel's equations. `cos2`—i.e., the cosine of the 
    /// angle between the normal and the transmitted ray—is wrapped in 
    /// an `Option` because it does not exist if the angle of incidence 
    /// is larger than the critical angle.
    fn cos_and_n(&self, ray: Ray, normal: Vector3D )->(Float, Float, Float, Option<Float>){
        let vin = ray.geometry.direction;

        let cos1 = vin*normal;
        let n1 = ray.refraction_index;
        let mut n2 = self.refraction_index;
        // If the ray already has this refraction index, assume
        // we are leaving a volume
        if (n1 - n2).abs() < 1e-7 {
            n2 = 1.0;
        }
        // Calculate cos2
        let sin1 = (1. - cos1*cos1).clamp(0., Float::MAX);
        let sin2 = n1 * sin1/n2; // Snell's law
        if sin2 >= 1. {
            // Pure reflection... this does not make
            return (n1, cos1, n2, None)
        }

        let cos2 = (1. - sin2*sin2).clamp(0., Float::MAX);
        (n1, cos1, n2, Some(cos2))
    }

    fn bsdf(&self,normal: Vector3D, mut ray: Ray, rng: &mut RandGen)->(Ray, Float, bool){
        let (n1, cos1, n2, cos2) = self.cos_and_n(ray, normal);
        let ray_dir = ray.geometry.direction;        
        let mirror_dir = mirror_direction(ray_dir, normal);
        if let Some(cos2) = cos2 {
            // There is transmission and reflection
            let refl = fresnel_reflectance(n1, cos1, n2, cos2);
            let r : Float = rng.gen();
            if r <= refl {
                // reflection
                ray.geometry.direction = mirror_dir;
                (ray, refl/cos1.abs(), true)
            }else{
                // Transmission
                ray.geometry.direction = fresnel_transmission_dir(ray_dir, normal, n1, cos1, n2, cos2);
                let n_ratio2 = (n2/n1)*(n2/n1);
                (ray, (1. - refl)*n_ratio2/cos1.abs(), true)
            }
        }else{
            // reflection only... there is no transmission
            ray.geometry.direction = mirror_dir;
            (ray, 1./cos1.abs(), true)

        }
            
    }

    fn eval_bsdf(&self, normal: Vector3D, ray: Ray, vout: Vector3D)->Float{        

        let (n1, cos1, n2, cos2) = self.cos_and_n(ray, normal);
        let vin = ray.geometry.direction;
        let mirror_dir = mirror_direction(vin, normal);
        if let Some(cos2) = cos2 {
            // Reflection and transmission
            // Calculate directions
            let trans_dir = fresnel_transmission_dir(vin, normal, n1, cos1, n2, cos2);
            let refl = fresnel_reflectance(n1, cos1, n2, cos2);
            let trans = 1. - refl;

            if vout.is_parallel(mirror_dir){
                refl
            }else if vout.is_parallel(trans_dir){
                trans
            }else{
                0.0
            }
        }else{
            // There is only reflection...
            if vout.is_parallel(mirror_dir){
                1.
            } else {
                0.0
            }
        }
        
        

    }
}


/*************** */
/* PLASTIC/METAL */
/*************** */


/// Information required for modelling Radiance's Plastic and Metal
pub struct PlasticMetal {
    pub color: Spectrum,
    pub specularity: Float,
    pub roughness: Float,
}

impl PlasticMetal{
    fn bsdf(&self,normal: Vector3D, e1: Vector3D, e2: Vector3D, mut ray: Ray, rng: &mut RandGen)->(Ray,Float, bool){
        
        if self.specularity > 0. {
            unimplemented!()
        }else{

            // Probability
            const ONE_OVER_PI: Float = 1. / PI;
            let prob = ONE_OVER_PI;
        
            let local_dir = sample_cosine_weighted_horizontal_hemisphere(rng);        
            debug_assert!( (local_dir.length() - 1.).abs() < 1e-6, "Length was {}", local_dir.length());
            let (x,y,z) = local_to_world(e1, e2, normal, Point3D::new(0., 0., 0.), local_dir.x, local_dir.y, local_dir.z);
            let dir = Vector3D::new(x,y,z);
            ray.geometry.direction = dir;
            // debug_assert!( (dir.length() - 1.).abs() < 1e-4);
            (ray, prob, false)
        }
    }

    fn eval_bsdf(&self, _normal: Vector3D, _e1: Vector3D, _e2: Vector3D, _vin: Vector3D, _vout: Vector3D)->Float{        
        if self.specularity > 0. {
            unimplemented!()
        }else{            
            1. / PI
        }
    }
}


/************ */
/* MIRROR  */
/************ */

fn mirror_bsdf(mut ray: Ray, normal: Vector3D)->(Ray, Float, bool){
    let ray_dir = ray.geometry.direction;
    ray.geometry.direction = mirror_direction(ray_dir, normal);            
    debug_assert!( (ray.geometry.direction.length() - 1.).abs() < 1e-8);            
    (ray, 1., true)
}

fn eval_mirror_bsdf(normal: Vector3D, vin: Vector3D, vout: Vector3D)->Float{        
    let mirror = mirror_direction(vin, normal);
    if vout.is_parallel(mirror) {
        1.
    } else {
        0.
    }
}


/************ */
/* MATERIALS */
/************ */

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

    /// Samples the bsdf, returns a new direction, the probability and a boolean
    /// indicating whether this is a specular or a diffuse interaction
    pub fn sample_bsdf(&self, normal: Vector3D, e1: Vector3D, e2: Vector3D, ray: Ray, rng: &mut RandGen)->(Ray,Float, bool){

        debug_assert!( (ray.geometry.direction.length() - 1.).abs() < 1e-5, "Length was {}", ray.geometry.direction.length());
        debug_assert!( (e1*e2).abs() < 1e-8);
        debug_assert!( (e1*normal).abs() < 1e-8);
        debug_assert!( (e2*normal).abs() < 1e-8);
    
        match self {
            Self::Plastic(s)=>s.bsdf(normal, e1, e2, ray, rng),
            Self::Metal(s)=>s.bsdf(normal, e1, e2, ray, rng),
            Self::Light(_)=>panic!("Trying to build a BSDF for a Light material"),
            Self::Mirror(_)=>mirror_bsdf(ray, normal),
            Self::Dielectric(s)=>s.bsdf(normal, ray, rng),
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

    #[test]
    fn test_mirror_direction() {
        fn check(v: Vector3D, normal: Vector3D, mirror: Vector3D) -> Result<(), String> {
            let v = v.get_normalized();
            let normal = normal.get_normalized();
            let mirror = mirror.get_normalized();

            let found = mirror_direction(v, normal);
            if !(mirror - found).is_zero() {
                return Err(format!(
                    "Expected mirror direction was {} | found {}",
                    mirror, found
                ));
            }
            Ok(())
        }

        check(
            Vector3D::new(0., 0., 1.),
            Vector3D::new(0., 0., 1.),
            Vector3D::new(0., 0., -1.),
        )
        .unwrap();
        check(
            Vector3D::new(0., 0., -1.),
            Vector3D::new(0., 0., -1.),
            Vector3D::new(0., 0., 1.),
        )
        .unwrap();
        check(
            Vector3D::new(1., 0., -1.).get_normalized(),
            Vector3D::new(0., 0., 1.),
            Vector3D::new(1., 0., 1.),
        )
        .unwrap();
    }

    fn get_vectors(rng: &mut RandGen) -> (Vector3D,Vector3D,Vector3D,Ray,Vector3D) {        
        let e1 = Vector3D::new(rng.gen(), rng.gen(), rng.gen()).get_normalized();
        let e2 = e1.get_perpendicular().unwrap();
        let normal = e1.cross(e2);
        let ray = Ray{
            geometry: geometry3d::Ray3D{
                direction: geometry3d::Vector3D::new(rng.gen(), rng.gen(), rng.gen()).get_normalized(),
                origin: geometry3d::Point3D::new(rng.gen(), rng.gen(), rng.gen())
            },
            refraction_index: rng.gen(),
        };
        let vout = Vector3D::new(1., 4., 12.).get_normalized();

        (normal, e1, e2, ray, vout)
    }

    fn test_material(material: Material){

        let mut rng = crate::rand::get_rng();
        for _ in 0..99999999{
            let (normal, e1, e2, ray, vout) = get_vectors(&mut rng);
            let (new_ray, pdf, _is_specular ) = material.sample_bsdf(normal, e1, e2, ray, &mut rng);
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

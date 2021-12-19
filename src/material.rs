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

use crate::{Float,PI};
use crate::colour::Spectrum;
use crate::samplers::{sample_cosine_weighted_horizontal_hemisphere, local_to_world};
use geometry3d::{Vector3D, Point3D};
use crate::rand::RandGen;

fn mirror_direction(vin: Vector3D, normal: Vector3D) -> Vector3D {
    debug_assert!((vin.length() - 1.).abs() < 1e-6);
    debug_assert!((normal.length() - 1.).abs() < 1e-6);
    let mut ret = vin - normal * (2. * (vin * normal));
    ret.normalize();
    ret
}

pub struct PlasticMetal {
    pub color: Spectrum,
    pub specularity: Float,
    pub roughness: Float,
}

impl PlasticMetal{
    fn bsdf(&self,normal: Vector3D, e1: Vector3D, e2: Vector3D, _ray_dir: Vector3D, rng: &mut RandGen)->(Vector3D,Float, bool){
        
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
            // debug_assert!( (dir.length() - 1.).abs() < 1e-4);
            (dir, prob, false)
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


fn mirror_bsdf(ray_dir: Vector3D, normal: Vector3D)->(Vector3D, Float, bool){
    let dir = mirror_direction(ray_dir, normal);            
    debug_assert!( (dir.length() - 1.).abs() < 1e-8);            
    (dir, 1., true)
}

fn eval_mirror_bsdf(normal: Vector3D, vin: Vector3D, vout: Vector3D)->Float{        
    let mirror = mirror_direction(vin, normal);
    if vout.is_parallel(mirror) {
        1.
    } else {
        0.
    }
}


pub enum Material {
    Plastic(PlasticMetal),
    Metal(PlasticMetal),
    Light(Spectrum),
    Mirror(Spectrum)
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

    // /// Gets an iterator that samples the BSDF of the material
    // /// according to a certain `ShadingInfo`. The closure receives a `Vector3D` corresponding
    // /// to the incident direction in world's coordinates, and returns the outgoing direction
    // /// as well as the probability of going in that direction.
    // /// 
    // /// # Note
    // /// 
    // /// When implementing forward/backard ray-tracing, modifications will have to be made 
    // /// to this function.
    // fn bsdf_sampler(&self, shading_info: ShadingInfo) -> Box<dyn Fn(Vector3D, &mut RandGen)->(Vector3D,Float)>;

    // /// Gets a closure that allows calculating and sampling the BSDF of the material
    // /// according to a certain `ShadingInfo`. The closure receives two `Vector3D`s, corresponding
    // /// to the incident and outgoing directions in world's coordinates, and returns the 
    // /// value of the BSDF accordingly.
    // /// 
    // /// # Note
    // /// 
    // /// When implementing forward/backard ray-tracing, modifications will have to be made 
    // /// to this function.
    // fn bsdf_evaluator(&self, shading_info: ShadingInfo) -> Box<dyn Fn(Vector3D, Vector3D)->Float>;

    
    /// Does this material scatter (e.g., like [`Plastic`]) or does it
    /// only transmit/reflects specularly (e.g., like [`Mirror`])?
    ///
    /// Defaults to `false`
    pub fn specular_only(&self) -> bool {
        match self{            
            Self::Mirror(_)=>true,
            _ => false
        }
    }

    /// Samples the bsdf, returns a new direction, the probability and a boolean
    /// indicating whether this is a specular or a diffuse interaction
    pub fn sample_bsdf(&self, normal: Vector3D, e1: Vector3D, e2: Vector3D, ray_dir: Vector3D, rng: &mut RandGen)->(Vector3D,Float, bool){
        debug_assert!( (ray_dir.length() - 1.).abs() < 1e-5, "Length was {}", ray_dir.length());
        debug_assert!( (e1*e2).abs() < 1e-8);
        debug_assert!( (e1*normal).abs() < 1e-8);
        debug_assert!( (e2*normal).abs() < 1e-8);
    
        match self{
            Self::Plastic(s)=>s.bsdf(normal, e1, e2, ray_dir, rng),
            Self::Metal(s)=>s.bsdf(normal, e1, e2, ray_dir, rng),
            Self::Light(_)=>panic!("Trying to build a BSDF for a Light material"),
            Self::Mirror(_)=>mirror_bsdf(ray_dir, normal),
        }
    }

    pub fn eval_bsdf(&self, normal: Vector3D, e1: Vector3D, e2: Vector3D, vin: Vector3D, vout: Vector3D)->Float{
        debug_assert!( (vin.length() - 1.).abs() < 1e-5, "Length was {}", vin.length());
        debug_assert!( (e1*e2).abs() < 1e-5);
        debug_assert!( (e1*normal).abs() < 1e-5);
        debug_assert!( (e2*normal).abs() < 1e-5);
        match self{
            Self::Plastic(s)=>s.eval_bsdf(normal, e1, e2, vin, vout),
            Self::Metal(s)=>s.eval_bsdf(normal, e1, e2, vin, vout),
            Self::Light(_)=>panic!("Trying to evaluate a BSDF for a Light material"),
            Self::Mirror(_)=>eval_mirror_bsdf(normal, vin, vout),
        }
    }
}


// pub struct Light {
//     pub red: Float,
//     pub green: Float,
//     pub blue: Float,
// }
// impl Material for Light {
//     fn colour(&self) -> Spectrum {
//         Spectrum {
//             red: self.red,
//             green: self.green,
//             blue: self.blue,
//         }
//     }

//     fn emits_direct_light(&self) -> bool {
//         true
//     }

//     fn emits_light(&self) -> bool {
//         true
//     }

//     // Lights don't reflect...?
//     fn bsdf_sampler(&self, _: ShadingInfo) -> Box<dyn Fn(Vector3D, &mut RandGen)-> (Vector3D,Float)> {
//         Box::new(|_vin: Vector3D, _rng: &mut RandGen|{
//             panic!("Trying to build a BSDF for a Light material")
//         })
//     }

//     fn bsdf_evaluator(&self, _: ShadingInfo) -> Box<dyn Fn(Vector3D, Vector3D)-> Float> {
//         Box::new(|_vin: Vector3D, _vout: Vector3D|{
//             0.0
//         })
//     }

    
// }


// pub struct Metal {
//     pub red: Float,
//     pub green: Float,
//     pub blue: Float,
//     pub specularity: Float,
//     pub roughness: Float,
// }

// impl Material for Metal {
//     fn colour(&self) -> Spectrum {
//         Spectrum {
//             red: self.red,
//             green: self.green,
//             blue: self.blue,
//         }
//     }

//     // Assume lambertian, for now
//     fn bsdf_sampler(&self, shading_info: ShadingInfo) -> Box<dyn Fn(Vector3D, &mut RandGen)-> (Vector3D, Float)> {
        
//         let normal = shading_info.normal.get_normalized();
//         let e1 = shading_info.dpdu.get_normalized();
//         let e2 = e1.cross(normal).get_normalized();
//         Box::new(move |_vin: Vector3D, rng: &mut RandGen|{    
//             // Probability
//             const ONE_OVER_PI: Float = 1. / PI;
//             let prob = ONE_OVER_PI;

//             let local_dir = sample_cosine_weighted_horizontal_hemisphere(rng);
//             debug_assert!( (local_dir.length() - 1.).abs() < 1e-8);
//             debug_assert!( (e1*e2).abs() < 1e-8);
//             debug_assert!( (e1*normal).abs() < 1e-8);
//             debug_assert!( (e2*normal).abs() < 1e-8);

//             let (x,y,z) = local_to_world(e1, e2, normal, Point3D::new(0., 0., 0.), local_dir.x, local_dir.y, local_dir.z);
//             let dir = Vector3D::new(x,y,z);
//             // debug_assert!( (dir.length() - 1.).abs() < 1e-4);
//             (dir, prob)
//         })
//     }

    
//     fn bsdf_evaluator(&self, _shading_info: ShadingInfo) -> Box<dyn Fn(Vector3D, Vector3D)-> Float> {
//         Box::new(|_vin: Vector3D, _vout: Vector3D|{            
//             // Probability
//             const ONE_OVER_PI: Float = 1. / PI;
//             ONE_OVER_PI
//         })
//     }
    

    
// }


// pub struct Plastic {
//     pub red: Float,
//     pub green: Float,
//     pub blue: Float,
//     pub specularity: Float,
//     pub roughness: Float,
// }

// impl Material for Plastic {
//     fn colour(&self) -> Spectrum {
//         Spectrum {
//             red: self.red,
//             green: self.green,
//             blue: self.blue,
//         }
//     }

//     // Assume lambertian, for now
//     fn bsdf_sampler(&self, shading_info: ShadingInfo) -> Box<dyn Fn(Vector3D, &mut RandGen)-> (Vector3D, Float)> {
        
//         let normal = shading_info.normal.get_normalized();
//         let e1 = shading_info.dpdu.get_normalized();
//         let e2 = e1.cross(normal).get_normalized();
//         Box::new(move |_vin: Vector3D, rng: &mut RandGen|{    
//             // Probability
//             const ONE_OVER_PI: Float = 1. / PI;
//             let prob = ONE_OVER_PI;

//             let local_dir = sample_cosine_weighted_horizontal_hemisphere(rng);            
//             debug_assert!( (local_dir.length() - 1.).abs() < 1e-5);
//             debug_assert!( (e1*e2).abs() < 1e-8);
//             debug_assert!( (e1*normal).abs() < 1e-8);
//             debug_assert!( (e2*normal).abs() < 1e-8);

//             let (x,y,z) = local_to_world(e1, e2, normal, Point3D::new(0., 0., 0.), local_dir.x, local_dir.y, local_dir.z);
//             let dir = Vector3D::new(x,y,z);
//             // debug_assert!( (dir.length() - 1.).abs() < 1e-4);
//             (dir, prob)
//         })
//     }

//     fn bsdf_evaluator(&self, _shading_info: ShadingInfo) -> Box<dyn Fn(Vector3D, Vector3D)-> Float> {
//         Box::new(|_vin: Vector3D, _vout: Vector3D|{            
//             // Probability
//             const ONE_OVER_PI: Float = 1. / PI;
//             ONE_OVER_PI
//         })
//     }
    
// }


// pub struct Mirror {
//     pub red: Float,
//     pub green: Float,
//     pub blue: Float,
// }

// impl Material for Mirror {
//     fn colour(&self) -> Spectrum {
//         Spectrum {
//             red: self.red,
//             green: self.green,
//             blue: self.blue,
//         }
//     }

    
//     fn bsdf_sampler(&self, shading_info: ShadingInfo) -> Box<dyn Fn(Vector3D, &mut RandGen)-> (Vector3D,Float)> {
//         let normal = shading_info.normal.get_normalized();
//         Box::new(move |vin: Vector3D, _rng: &mut RandGen|{                    
//             let dir = mirror_direction(vin, normal);
            
//             debug_assert!( (dir.length() - 1.).abs() < 1e-8);            
//             (dir, 1.)
//         })
//     }

    
//     fn bsdf_evaluator(&self, shading_info: ShadingInfo) -> Box<dyn Fn(Vector3D, Vector3D)-> Float> {
//         let normal = shading_info.normal.get_normalized();
//         Box::new(move |vin: Vector3D, vout: Vector3D|{            
//             let mirror = mirror_direction(vin, normal);
//             // All of it goes to the mirror direction
//             if vout.is_parallel(mirror) {
//                 1.
//             } else {
//                 0.
//             }
//         })
//     }
    

    
// }

// // pub struct Dielectric{
// //     pub red: Float,
// //     pub green: Float,
// //     pub blue: Float,
// //     pub refraction_coefficient: Float,
// // }

// // impl Dielectric {

// // }

// // impl Material for Dielectric{
// //     fn colour(&self) -> Spectrum {
// //         Spectrum {
// //             red: self.red,
// //             green: self.green,
// //             blue: self.blue,
// //         }
// //     }

// //     fn bsdf(&self, vin: Vector3D, normal: Vector3D, vout: Vector3D) -> Spectrum{

// //     }

// //     fn specular_only(&self)->bool{
// //         false
// //     }
// // }

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
}

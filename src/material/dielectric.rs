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


use crate::Float;
use crate::colour::Spectrum;
use crate::ray::Ray;
use crate::rand::*;
use geometry3d::{Vector3D, Point3D};
use crate::material::specular::*;


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
    /// 
    /// # Example
    /// ```
    /// use geometry3d::{Point3D,Vector3D, Ray3D};
    /// use rendering::colour::Spectrum;
    /// use rendering::material::Dielectric;
    /// use rendering::ray::Ray;
    /// let mat = Dielectric{
    ///     color: Spectrum::gray(0.23), //irrelevant for this test
    ///     refraction_index: 1.52
    /// };
    /// let normal = Vector3D::new(0., 0., 1.);
    /// let ray = Ray{
    ///     geometry: Ray3D{
    ///         origin: Point3D::new(0., 0., 1.),
    ///         direction: Vector3D::new(0., 1., -2.).get_normalized()
    ///     },
    ///     refraction_index : 1.
    /// };
    /// let (n1, cos1, n2, cos2) = mat.cos_and_n(ray, normal);
    /// ```
    pub fn cos_and_n(&self, ray: Ray, normal: Vector3D )->(Float, Float, Float, Option<Float>){
        let vin = ray.geometry.direction;

        let cos1 = (vin*normal).abs();
        let n1 = ray.refraction_index;
        let mut n2 = self.refraction_index;
        // If the ray already has this refraction index, assume
        // we are leaving a volume, entering air.
        if (n1 - n2).abs() < 1e-7 {
            n2 = 1.0;
            // std::mem::swap(&mut n1, &mut n2);
        }
        // Calculate cos2
        let sin1_sq = (1. - cos1*cos1).clamp(0., Float::MAX);
        let sin2_sq = n1 * n1 * sin1_sq / (n2*n2); // Snell's law squared
        #[cfg(debug_assertions)]
        {
            let lhs = n1*sin1_sq.sqrt();
            let rhs = n2*sin2_sq.sqrt();
            debug_assert!( (lhs - rhs).abs() < 1e-5, "rhs = {}, lhs = {}", lhs, rhs );            
        }
        if sin2_sq >= 1. {
            // Pure reflection...  
            return (n1, cos1, n2, None)
        }
        

        let cos2 = (1. - sin2_sq).sqrt();
        
        (n1, cos1, n2, Some(cos2))
    }

    /// Gets the Reflected and Transmitted BSDF values
    pub fn refl_trans(&self, n1: Float, cos1: Float, n2: Float, cos2: Option<Float>)->(Float, Float){
        
        if let Some(cos2) = cos2 {
            // There is refraction
            let refl = fresnel_reflectance(n1, cos1, n2, cos2);            
            let refl_comp = refl/cos1.abs();
            let ratio = n2/n1;
            let n_ratio2 = ratio*ratio;
            let t_comp = (1. - refl)*n_ratio2/cos1.abs();
            (refl_comp, t_comp)
        }else{
            // pure reflection            
            (1./cos1,0.)            
        }

        
    }
    

    pub fn bsdf(&self,normal: Vector3D, intersection_pt: Point3D, mut ray: Ray, rng: &mut RandGen)->(Ray, Float, bool){
        let (n1, cos1, n2, cos2) = self.cos_and_n(ray, normal);
        let (refl, trans) = self.refl_trans(n1, cos1, n2, cos2);
        let ray_dir = ray.geometry.direction;        
        let mirror_dir = mirror_direction(ray_dir, normal);
        debug_assert!((1.- mirror_dir.length()).abs() < 1e-5, "length is {}", mirror_dir.length());
        
        
        let r : Float = rng.gen();
        if r <= refl/(refl + trans) {
            // Reflection            
            // avoid self shading
            ray.geometry.origin = intersection_pt + normal*0.00001;

            ray.geometry.direction = mirror_dir;
            (ray, refl, true)                        
        }else{ 
            // Transmission                        
            // avoid self shading            
            ray.geometry.origin = intersection_pt - normal*0.00001;
            
            ray.refraction_index = n2;
            let trans_dir = fresnel_transmission_dir(ray_dir, normal, n1, cos1, n2, cos2.unwrap());            
            ray.geometry.direction = trans_dir;                
            (ray, trans, true)            
            
        }
                    
    }

    pub fn eval_bsdf(&self, normal: Vector3D, ray: Ray, vout: Vector3D)->Float{        

        let (n1, cos1, n2, cos2) = self.cos_and_n(ray, normal);
        let (refl, trans) = self.refl_trans(n1, cos1, n2, cos2);
        let vin = ray.geometry.direction;
        let mirror_dir = mirror_direction(vin, normal);
        debug_assert!((1.- mirror_dir.length()).abs() < 1e-5, "length is {}", mirror_dir.length());
        
        // If reflection
        if vout.is_same_direction(mirror_dir){
           return refl
        } 
        
        // Check transmission
        if let Some(cos2) = cos2 {
            // There is Reflection and transmission
            let trans_dir = fresnel_transmission_dir(vin, normal, n1, cos1, n2, cos2);
            debug_assert!((1.- trans_dir.length()).abs() < 1e-5, "length is {}", trans_dir.length());            
            if vout.is_same_direction(trans_dir){
                // transmission
                return trans
            }
        }

        // Neither... 
        0.0
        
        

    }
}








#[cfg(test)]
mod tests {
    use super::*;


    use crate::ray::Ray;
    use geometry3d::{Point3D, Ray3D};
    #[test]
    fn test_normal_incidence(){

        // Example found online, a glass_air transition
        let n1 = 1.52; // glass
        let n2 = 1.; // air

        let normal = Vector3D::new(0., 0., 1.);
        
        let mat = Dielectric{
            color: Spectrum::gray(0.1), //irrelevant for this test
            refraction_index: n2
        };

        // Perpendicular rays aren't deviated
        let ray = Ray{
            refraction_index: n1,
            geometry: Ray3D{
                origin: Point3D::new(0., 0., 10.),
                direction: Vector3D::new(0., 0., -1.)
            }
        };

        let (np1, cos1, np2, cos2) = mat.cos_and_n(ray, normal);        
        assert!( (n1-np1).abs() < 1e-8, "np1 = {}, n1 = {}", np1, n1);
        assert!( (n2-np2).abs() < 1e-8, "np2 = {}, n2 = {}", np2, n2);
        assert!((1. - cos1).abs() < 1e-8, "cos1 = {}", cos1);
        assert!(cos2.is_some());
        let cos2 = cos2.unwrap();
        assert!((1. - cos2).abs() < 1e-8, "cos2 = {}", cos2);

        
    }

    #[test]
    fn test_critical_angle(){

        // Example found online, a glass_air transition
        let n1 = 1.52 as Float; // glass
        let n2 = 1.003 as Float; // air
        
        let normal = Vector3D::new(0., 0., 1.);
        
        let mat = Dielectric{
            color: Spectrum::gray(0.1), //irrelevant for this test
            refraction_index: n2
        };
        
        let crit = (n2/n1).asin();
        

        let direction = |angle: Float|->Vector3D{
            let direction = Vector3D::new(0., angle.sin(), -angle.cos());            
            let found_angle = (direction*normal).abs().acos();
            assert!( (found_angle-angle).abs() < 1e-7, "angle = {} | found_angle = {}", angle, found_angle);
            direction
        };

        // Check before critical angle
        let mut angle = 0.;
        let angle_d = 0.1;
        while angle < crit {
            let ray = Ray{
                refraction_index: n1,
                geometry: Ray3D{
                    origin: Point3D::new(0., 0., 10.),
                    direction: direction(angle.to_radians())
                }
            };

            let (_np1, _cos1, _np2, cos2) = mat.cos_and_n(ray, normal);                    
            assert!(cos2.is_some());
            angle += angle_d;
        }

        // Check critical angle
        angle = crit;
        let ray = Ray{
            refraction_index: n1,
            geometry: Ray3D{
                origin: Point3D::new(0., 0., 10.),
                direction: direction(angle.to_radians())
            }
        };

        let (_np1, _cos1, _np2, cos2) = mat.cos_and_n(ray, normal);                    
        assert!(cos2.is_some());
        angle += angle_d;

        // Check beyond critical angle
        while angle < crit {
            let ray = Ray{
                refraction_index: n1,
                geometry: Ray3D{
                    origin: Point3D::new(0., 0., 10.),
                    direction: direction(angle.to_radians())
                }
            };

            let (_np1, _cos1, _np2, cos2) = mat.cos_and_n(ray, normal);                    
            assert!(cos2.is_some());
            angle += angle_d;
        }
        
    }

    #[test]
    fn test_sin_cos_n(){
        let n = 1.52;
        let mat = Dielectric{
            color: Spectrum::gray(0.23), //irrelevant for this test
            refraction_index: n
        };

        let normal = Vector3D::new(0., 0., 1.);

        let dir_zero = Vector3D::new(0., 1., -2.).get_normalized(); // going down

        let ray = Ray {
            geometry: Ray3D{
                origin: Point3D::new(0., 0., 0.),
                direction: dir_zero
            },
            refraction_index: 2.9
        };

        let (n1, cos1, n2, cos2) = mat.cos_and_n(ray, normal);
        let theta1 = cos1.acos();
        let theta2 = cos2.unwrap().acos();
        let fresnel_1 = n1*theta1.sin();
        let fresnel_2 = n2*theta2.sin();

        assert!( (fresnel_1 - fresnel_2).abs() < 1e-5, "n1*sin1 = {} | n2*sin2 = {}", fresnel_1, fresnel_2);
    }

    #[test]
    fn test_bsdf(){
        let n = 1.52;
        let mat = Dielectric{
            color: Spectrum::gray(0.23), //irrelevant for this test
            refraction_index: n
        };

        let mut rng = get_rng();
        let normal = Vector3D::new(0., 0., 1.);

        let dir_zero = Vector3D::new(0., 1., -2.).get_normalized(); // going down

        let mut ray = Ray {
            geometry: Ray3D{
                origin: Point3D::new(0., 0., 0.),
                direction: dir_zero
            },
            refraction_index: 1.
        };
        println!("Before entering: {}", dir_zero);
        let mirror_dir = mirror_direction(ray.geometry.direction, normal);        
        let mut trans_dir : Option<Vector3D> = None;
        // Get INTO the material                
        for _ in 0..30 {
            let (new_ray, _pdf, _is_specular) = mat.bsdf(normal, Point3D::new(0., 0., 0.), ray, &mut rng);
            println!("A -- PDF = {}", _pdf);
            let new_dir = new_ray.geometry.direction;
            if new_dir.z < 0. {
                // We are still moving down... thus, refraction
                assert!(new_ray.refraction_index == n, "Expeting n={}, found n={}", n, new_ray.refraction_index);
                trans_dir = Some(new_dir);
            } else {
                // reflection
                assert!(new_dir.is_same_direction(mirror_dir));                
            }
        }

        println!("Inside: {:?}", trans_dir);
        
        // Get OUT OF the material  
        ray.refraction_index = n;    
        ray.geometry.direction = trans_dir.unwrap();  
        for _ in 0..30 {
            let (new_ray, _pdf, _is_specular) = mat.bsdf(normal, Point3D::new(0., 0., 0.), ray, &mut rng);
            println!("B -- PDF = {}", _pdf);
            let new_dir = new_ray.geometry.direction;
            if new_dir.z < 0. {
                // We are still moving down... thus, refraction
                assert!(new_ray.refraction_index == 1., "Expeting n={}, found n={}", 1, new_ray.refraction_index);
                assert!(new_dir.is_same_direction(dir_zero), "ray_dir = {} | new_dir = {} | dir_zero = {}", ray.geometry.direction, new_dir, dir_zero);                
                println!("After leaving : {}", new_dir);
            }             
        }
    }

    // use crate::scene::Scene;
    // use crate::primitive::Primitive;
    // use crate::material::Material;
    // use crate::interaction::Interaction;
    // use geometry3d::{Sphere3D};
    // use geometry3d::intersection::{SurfaceSide};
    
    // #[test]
    // fn debug_dielectric(){
    //     let mut scene = Scene::default();

    //     let n = 1.52;
    //     let radius = 1.;
    //     let glass = scene.push_material(Material::Dielectric(Dielectric{
    //         color: Spectrum::gray(0.23), //irrelevant for this test
    //         refraction_index: n
    //     }));

    //     scene.push_object(
    //         glass,
    //         glass,
    //         Primitive::Sphere(Sphere3D::new(radius, Point3D::new(0.,0.,0.)))
    //     );

    //     scene.build_accelerator();
    //     let mut rng = crate::rand::get_rng();

    //     let ray = Ray{
    //         geometry: Ray3D{
    //             origin: Point3D::new(0., -10., 0.15),
    //             direction: Vector3D::new(0., 1., 0.)
    //         },
    //         refraction_index: 1.
    //     };

    //     if let Some((distance, Interaction::Surface(data)))= scene.cast_ray(&ray){
    //         let object = &scene.objects[data.prim_index];
    //         let normal = data.geometry_shading.normal;
    //         let e1 = data.geometry_shading.dpdu.get_normalized();
    //         let e2 = normal.cross(e1);//.get_normalized();
    //         let material = match data.geometry_shading.side {
    //             SurfaceSide::Front => {
    //                 &scene.materials[object.front_material_index]
    //             },
    //             SurfaceSide::Back =>{
    //                 &scene.materials[object.back_material_index]
    //             },
    //             SurfaceSide::NonApplicable => {
    //                 // Hit parallel to the surface
    //                 panic!("Wrong intersection")
    //             }                    
    //         };
    //         let intersection_pt = ray.geometry.project(distance); 
                                               
    //         loop {
    //             let (mut new_ray, _material_pdf,  _is_specular) = material.sample_bsdf(normal, e1, e2, Point3D::new(0., 0., 0.), ray, &mut rng);                            
    //             if new_ray.geometry.direction.y > 0. {
    //                 // if refracted.
    //                 new_ray.geometry.origin = intersection_pt + normal*0.00001;
    //                 assert!( (new_ray.refraction_index - n).abs() < 1e-5, "new n = {}, n = {}",new_ray.refraction_index, n );

    //             }

    //         }

    //     }else{
    //         panic!("Did not intersect sphere")
    //     }
    // }

}


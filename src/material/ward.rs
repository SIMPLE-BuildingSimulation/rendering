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

use crate::{Float, PI};
use geometry3d::{Vector3D,Point3D};
use crate::rand::*;
use crate::ray::Ray;

use crate::samplers::{local_to_world,sample_cosine_weighted_horizontal_hemisphere};


/// Samples a Ward BSDF, changing the direction of a given Ray. Returns a tuple with the 
/// Specular and Diffuse reflections, as well as the Weighting factor for the BSDF
/// 
/// This implementation is based on "A new Ward BRDF model with bounded albedo" (2010), 
/// by David Geisler-Moroder and Arne Dür
pub fn sample_ward_anisotropic(
    normal: Vector3D,
    e1: Vector3D,
    e2: Vector3D,
    intersection_pt: Point3D,
    specularity: Float, 
    alpha: Float,
    beta: Float,
    ray: &mut Ray,
    rng: &mut RandGen,    
)-> (Float, Float, Float) {

    ray.geometry.origin = intersection_pt + normal * 0.00001;

    let (prob_spec, u, v): (Float, Float, Float) = rng.gen();

    

    if prob_spec < specularity  {
        // incident direction
        let l = ray.geometry.direction*-1.;        
        
        // Make sure that phi_h falls in the same quadrant as 2*pi*v;
        let phi_h: Float;
        if (0.25 - v).abs() < 1e-6 {
            phi_h = PI/2.;
        }else if (0.75 - v).abs() < 1e-6 {
            phi_h = PI/2. + PI;
        }else if v < 0.25 {
            phi_h = ((beta/alpha)*(2.*PI*v).tan()).atan();
        }else if v < 0.75 {
            phi_h = ((beta/alpha)*(2.*PI*v).tan()).atan() + PI;
        }else{
            phi_h = ((beta/alpha)*(2.*PI*v).tan()).atan() + 2.*PI;
        }
        
        
        let cos_phi = phi_h.cos();
        let sin_phi = phi_h.sin();

        // let phi_h = ((alpha_y/alpha_x)*(2.*PI*v)).atan2(other..?);
        let theta_h = ( -u.ln() / ((cos_phi/alpha).powi(2)+(sin_phi/beta).powi(2))  ).sqrt().atan();
        let sin_theta = theta_h.sin();
        let cos_theta = theta_h.cos();
        let h = Vector3D::new(
            sin_theta * cos_phi,
            sin_theta * sin_phi,
            cos_theta
        );

        // reflected direction
        let v = h * (h*l)*2. - l;                                    
        let l_n = l*normal;
        let l_h =  l*h;
        let v_n = v*normal;
        let v_h = h*v;
        if v_n < 0.0 || l_n < 0.0 {
            // This should not happen... I am not sure
            // whether this is an error in Walter's "Notes on the Ward BRDF" paper... or 
            // my own error.
            return (0.0, 0., 1.0)
        }            
        let h_n = h*normal;
        
        let h = l+v;            
        
        // Eq. 11
        // let c1 = specularity  / ( PI*alpha*beta* 4.*l_h.powi(2) * h_n.powi(4) );  
        // let c2 = -( (h*e1/alpha).powi(2) + (h*e2/beta).powi(2) )/( h_n.powi(2) );
        // let spec = c1*c2.exp() ;
        

        // Eq. 17
        let c1 = specularity  / ( PI*alpha*beta * h_n.powi(4) );  
        let c2 = -( (h*e1/alpha).powi(2) + (h*e2/beta).powi(2) )/( h_n.powi(2) );
        let spec = c1*c2.exp() ;
                    
        
        let diffuse = (1. - specularity) / PI;   

        let weight = 2./(1. + v_n/l_n); // Eq. 15        
        if spec.is_nan(){
            panic!("incorrect (i.e., NaN) bsdf when calculating Ward aniso.");
            
        }
        ray.geometry.direction = v ;            
        return ( spec, diffuse, weight)// mark as specular
        
    } else {
        // Probability
        
        let local_dir = sample_cosine_weighted_horizontal_hemisphere(rng);                            
        let diffuse = (1. - specularity)/PI;
        let weight = PI;


        let (x, y, z) = local_to_world(
            e1,
            e2,
            normal,
            Point3D::new(0., 0., 0.),
            local_dir.x,
            local_dir.y,
            local_dir.z,
        );
        
        ray.geometry.direction = Vector3D::new(x, y, z);
        ( 0.0, diffuse, weight)
        
        
    }

        
}


/// Evaluates a Ward BSDF, changing the direction of a given Ray. Returns a tuple with the 
/// Specular and Diffuse reflections
/// 
/// This implementation is based on "A new Ward BRDF model with bounded albedo" (2010), 
/// by David Geisler-Moroder and Arne Dür
pub fn evaluate_ward_anisotropic(
    normal: Vector3D,
    e1: Vector3D,
    e2: Vector3D,
    specularity: Float, 
    alpha: Float,
    beta: Float,
    ray: &Ray,
    o: Vector3D,
)->(Float, Float){
    
        let o_n = o*normal;


        // Light is behind the surface
        if o_n < 1e-5 {
            return (0.0, 0.0)
        }

        let spec = if specularity > 1e-5 && (alpha > 1e-5 || beta > 1e-5 ){
            // Don't bother calculating the specular part if there is no roughness... it won't contribute            
            let i = ray.geometry.direction*-1.;
            // what if alphas are zero?
            let h = i+o;
            #[cfg(debug_assertions)]
            {
                let i_n = i*normal;
                
                if i_n < 0.{
                    debug_assert!(i_n > 0.0, "i*n = {}", i_n);
                }
            }
                        
            let h_n = h*normal;
            // Eq. 17
            let c1 = specularity  / ( PI*alpha*beta * h_n.powi(4) );  
            let c2 = -( (h*e1/alpha).powi(2) + (h*e2/beta).powi(2) )/( h_n.powi(2) );
            c1*c2.exp() 
                        
        }else{
            0.0
        };

        (spec, (1. - specularity) / PI)
}
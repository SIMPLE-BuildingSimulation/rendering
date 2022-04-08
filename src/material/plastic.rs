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
use crate::material::Material;
use crate::samplers::{local_to_world, sample_cosine_weighted_horizontal_hemisphere};

/// Information required for modelling Radiance's Plastic and Plastic
pub struct Plastic {
    pub colour: Spectrum,
    pub specularity: Float,
    pub roughness: Float,
}

impl Material for Plastic {
    
    fn id(&self)->&str{
        "Plastic"
    }

    fn colour(&self) -> Spectrum{
        self.colour
    }

    /// A New Ward BRDF Model with Bounded Albedo -- Moroder et al
    fn sample_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        intersection_pt: Point3D,
        ray: &mut Ray,
        rng: &mut RandGen,
    ) -> ( Float, Float, bool) {
        
        ray.geometry.origin = intersection_pt + normal * 0.00001;

        let (prob_spec, u, v): (Float, Float, Float) = rng.gen();

        let (bsdf, prob, is_speculer, local_dir) : (Float, Float, bool, Vector3D);

        if prob_spec < self.specularity  {
            let i = ray.geometry.direction*-1.;

            let alpha_x = self.roughness;
            let alpha_y = 12.*self.roughness; // what if alpha_x == 0?

            
            // Make sure that phi_h falls in the same quadrant as 2*pi*v;
            let phi_h: Float;
            if (0.25 - v).abs() < 1e-6 {
                phi_h = PI/2.;
            }else if (0.75 - v).abs() < 1e-6 {
                phi_h = PI/2. + PI;
            }else if v < 0.25 {
                phi_h = ((alpha_y/alpha_x)*(2.*PI*v).tan()).atan();
            }else if v < 0.75 {
                phi_h = ((alpha_y/alpha_x)*(2.*PI*v).tan()).atan() + PI;
            }else{
                phi_h = ((alpha_y/alpha_x)*(2.*PI*v).tan()).atan() + 2.*PI;
            }
            
            
            let cos_phi = phi_h.cos();
            let sin_phi = phi_h.sin();

            // let phi_h = ((alpha_y/alpha_x)*(2.*PI*v)).atan2(other..?);
            let theta_h = ( -u.ln() / ((cos_phi/alpha_x).powi(2)+(sin_phi/alpha_y).powi(2))  ).sqrt().atan();
            let sin_theta = theta_h.sin();
            let cos_theta = theta_h.cos();
            let h = Vector3D::new(
                sin_theta * cos_phi,
                sin_theta * sin_phi,
                cos_theta
            );

            let o = h * (h*i)*2. - i;                                    
            let i_n = i*normal;
            let o_n = o*normal;
            let h_o = h*o;
            if o_n < 0.0 || i_n < 0.0 {
                // This should not happen... I am not sure
                // whether this is an error in Walter's "Notes on the Ward BRDF" paper... or 
                // my own error.
                return (0., 1., true)
            }
            // let h_i = h*i;
            let h_n = h*normal;
            
            let h = i+o;
            // let c1 = self.specularity / ( 4.*PI*alpha_x*alpha_y*(i_n*o_n).sqrt() );            
            let c1 = self.specularity * h_o.powi(2)/ ( PI*alpha_x*alpha_y*h_n.powi(4) );            
            let c2 = -( (h*e1/alpha_x).powi(2) + (h*e2/alpha_y).powi(2) )/( h_n.powi(2) );
            
            // prob = self.specularity * h_i * h_n.powi(3)*(o_n/i_n).sqrt();
            prob = 2./(1. + i_n/o_n);
            ray.geometry.direction = o;
            // bsdf = c1*c2.exp();
            bsdf = c1*c2.exp() + (1. - self.specularity) / PI;
            is_speculer = true;
            if bsdf.is_nan(){
                println!("wrokng bsdf");
                return ( 0., 1., is_speculer)
            }
            return ( bsdf, prob, is_speculer)
            
        } else {
            // Probability
            
            local_dir = sample_cosine_weighted_horizontal_hemisphere(rng);                            
            bsdf = (1. - self.specularity)/PI;
            prob = 1./PI;
            is_speculer = false;
            
        }

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
        
        ray.geometry.direction = Vector3D::new(x, y, z);
        ( bsdf, prob, is_speculer)
    }

    
    fn eval_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        ray: &Ray,
        vout: Vector3D,
    ) -> Float{        
        let o = vout*-1.;
        let o_n = o*normal;

        if o_n < 1e-5 {
            return 0.0
        }

        let spec = if self.roughness > 1e-5{
            // Don't bother calculating the specular part if there is no roughness... it won't contribute
            let alpha_x = self.roughness;
            let alpha_y = self.roughness;
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
            // let c1 = self.specularity / ( 4.*PI*alpha_x*alpha_y*((i_n)*(o_n)).sqrt() );
            let c1 = self.specularity / ( 4.*PI*alpha_x*alpha_y*h_n.powi(4));            
            let c2 = -( (h*e1/alpha_x).powi(2) + (h*e2/alpha_y).powi(2) )/( (h*normal).powi(2) );
            c1*c2.exp()
            
                        
        }else{
            0.0
        };

        spec + (1. - self.specularity) / PI

        // } else {
            
        // }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use geometry3d::Ray3D;

    #[test]
    fn test_specular_plastic(){
        let plastic = Plastic{
            colour: Spectrum{red: 0.2, green: 0.2, blue: 0.2},
            specularity: 0.1, 
            roughness: 0.1,
        };

        let normal = Vector3D::new(0., 0., 1.);
        let e1= Vector3D::new(1., 0., 0.);
        let e2 = Vector3D::new(0., 1., 0.);
        let intersection_pt = Point3D::new(0., 0., 0.);

        let mut ray= Ray { 
            geometry: Ray3D{
                origin: Point3D::new(-1., 0., 1.),
                direction: Vector3D::new(1., 0., -1.).get_normalized(),
            }, 
            .. Ray::default()
        };

        let mut rng = crate::rand::get_rng();
            


        plastic.sample_bsdf(normal, e1, e2, intersection_pt, &mut ray, &mut rng);
    }
}


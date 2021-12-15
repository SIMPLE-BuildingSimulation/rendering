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
use geometry3d::{Point3D, Vector3D};
use crate::rand::*;

#[cfg(feature = "parallel")]
use rayon::iter::ParallelIterator;


pub fn uniform_sample_triangle(rng: &mut RandGen,a:Point3D,b:Point3D,c:Point3D)->Point3D{
    let (rand1, rand2): (Float, Float) = rng.gen();
    // let rand1 : Float = rng.gen();
    // let rand2 : Float = rng.gen();
    let aux = rand1.sqrt();
    let u = 1. - aux;
    let v = rand2*aux;
    let v1 = b-a;
    let v2 = c-a;
    // return
    a + v1 * u + v2 * v
}

pub struct HorizontalDiskUniformSampler{
    n_samples: usize,
    i: usize,
    rng: RandGen,
    radius: Float,
}

impl HorizontalDiskUniformSampler {
    pub fn new(radius: Float, n_samples: usize)->Self{
        Self{
            i: 0,
            rng: get_rng(),
            radius,
            n_samples,
        }
    }
}

impl Iterator for HorizontalDiskUniformSampler{
    type Item = (f32, f32);
    fn next(&mut self)->Option<Self::Item>{
        if self.n_samples == self.i {
            return None
        }
        self.i += 1;
        Some(uniform_sample_horizontal_disc(&mut self.rng, self.radius))
    }
}



pub fn uniform_sample_horizontal_disc(rng: &mut RandGen, radius: Float) -> (f32, f32) {
    // sqrt() and cos() and sin() are 
    // much faster in f32... that is why I am doing 
    // this.
    let (r, theta): (f32, f32) = rng.gen();
    
    let r = radius as f32 * r.sqrt();
    let theta = 2. * std::f32::consts::PI * theta;
    let (theta_sin, theta_cos) = theta.sin_cos();
    let local_x = r * theta_sin;
    let local_y = r * theta_cos;
    (local_x , local_y)
}

pub fn local_to_world(
    local_e1: Vector3D,
    local_e2: Vector3D,
    normal: Vector3D,
    centre: Point3D,
    x_local: Float,
    y_local: Float,
    z_local: Float,    

) -> (Float, Float, Float) {

    // Check that they are normalized
    debug_assert!((1. - local_e1.length_squared()).abs() < 1e-4);
    debug_assert!((1. - local_e2.length_squared()).abs() < 1e-4);
    debug_assert!((1. - normal.length_squared()).abs() < 1e-4);
    
    
    let x = centre.x + x_local * local_e1.x + y_local*local_e2.x + z_local * normal.x;
    let y = centre.y + x_local * local_e1.y + y_local*local_e2.y + z_local * normal.y;
    let z = centre.z + x_local * local_e1.z + y_local*local_e2.z + z_local * normal.z;

    (x, y, z)
}


pub struct HorizontalCosineWeightedHemisphereSampler{            
    disk_sampler: HorizontalDiskUniformSampler,
}

impl HorizontalCosineWeightedHemisphereSampler {
    pub fn new(n_samples: usize)->Self{
        Self{            
            disk_sampler: HorizontalDiskUniformSampler::new(1., n_samples),
        }
    }
}

impl Iterator for HorizontalCosineWeightedHemisphereSampler{
    type Item = Vector3D;
    fn next(&mut self)->Option<Self::Item>{
        if let Some((local_x, local_y)) = self.disk_sampler.next() {
            let local_z = (1. - local_x * local_x - local_y * local_y).sqrt();
            let ret = Vector3D::new(local_x as Float, local_y as Float, local_z as Float);
            Some(ret)
        }else{
            None
        }
    }
}


/// Gets a random `Vector3D`, distributed according to `cos(theta)` according
/// to a normal `Vector3D(0,0,1)`
pub fn sample_cosine_weighted_horizontal_hemisphere(rng: &mut RandGen) -> Vector3D {
    
    let (local_x, local_y) = uniform_sample_horizontal_disc(rng, 1.);
    let local_z = (1. - local_x * local_x - local_y * local_y).sqrt();
    Vector3D::new(local_x as Float, local_y as Float, local_z as Float)
}



pub fn uniform_sample_hemisphere(rng: &mut RandGen, e1: Vector3D, e2: Vector3D, normal: Vector3D) -> Vector3D {
    // Calculate in
    
    let (rand1, rand2): (f32, f32) = rng.gen();
    // let rand2: f32 = rng.gen();
    let sq  = (1.0 - rand1 * rand1).sqrt();
    let pie2  = 2.0 * std::f32::consts::PI * rand2;
    let (pie2_sin, pie2_cos) = pie2.sin_cos();
    let local_x = pie2_cos * sq;
    let local_y = pie2_sin * sq;
    let local_z = rand1;

    // Take back to world normal    
    let (x, y, z) = local_to_world(e1, e2, normal,Point3D::new(0., 0., 0.), local_x as Float, local_y as Float, local_z as Float);
    debug_assert!((Vector3D::new(x, y, z).length() - 1.).abs() < 0.0000001);
    Vector3D::new(x, y, z)
}

pub fn uniform_sample_disc(
    rng: &mut RandGen,
    radius: Float,
    centre: Point3D,
    normal: Vector3D,
) -> Point3D {
    let (x_local, y_local) = uniform_sample_horizontal_disc(rng, radius);

    // Form the basis
    let e2 = normal.get_perpendicular().unwrap();
    let e1 = e2.cross(normal);
    let (x, y, z) = local_to_world( e1,e2,normal,centre, x_local as Float, y_local as Float, 0.);
    Point3D::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_sample_disc() {
        fn check(radius: Float, centre: Point3D, normal: Vector3D) -> Result<(), String> {
            let mut rng = get_rng();
            let normal = normal.get_normalized();
            let p = uniform_sample_disc(&mut rng, radius, centre, normal);
            if ((p - centre) * normal).abs() > 100. * Float::EPSILON {
                return Err(format!(
                    "Point is not coplanar with circle. ((p-centre)*normal).abs() == {}",
                    ((p - centre) * normal).abs()
                ));
            }
            if (p - centre).length() > radius {
                return Err(format!(
                    "Sample out of circle. Point sampled was {} | p-centre = {} | radius = {}",
                    p,
                    (p - centre).length(),
                    radius
                ));
            }

            Ok(())
        }

        for _ in 0..100 {
            check(1.2, Point3D::new(0., 0., 0.), Vector3D::new(0., 0., 1.)).unwrap();
            check(4.2, Point3D::new(3., 0., 0.), Vector3D::new(0., 1., 1.)).unwrap();
            check(0.12, Point3D::new(0., 1., 0.), Vector3D::new(0., 1., 0.)).unwrap();
            check(23., Point3D::new(0., -10., -20.), Vector3D::new(1., 1., 0.)).unwrap();
            check(23., Point3D::new(0., -10., -20.), Vector3D::new(1., 0., 0.)).unwrap();
        }
    }

    #[test]
    fn test_uniform_sample_hemisphere() {
        fn check(normal: Vector3D) -> Result<(), String> {
            let normal = normal.get_normalized();
            let e2 = normal.get_perpendicular().unwrap();
            let e1 = e2.cross(normal);
            
            let mut rng = get_rng();
            let dir = uniform_sample_hemisphere(&mut rng, e1, e2, normal);

            if (1. - dir.length()).abs() > 1e-5 {
                return Err(format!("Sampled direction (from uniform_sample_hemisphere) was nor normalized... {} (length = {})", dir, dir.length()));
            }
            if dir * normal < 0. {
                return Err(format!("Sampled direction (from uniform_sample_hemisphere) is not in hemisphere... Normal = {} | Dir = {}", normal, dir));
            }

            Ok(())
        }

        for _ in 0..100 {
            check(Vector3D::new(1., 2., -1.)).unwrap();
            check(Vector3D::new(-1., 0., 0.)).unwrap();
            check(Vector3D::new(0., 0., 1.)).unwrap();
            check(Vector3D::new(0., 1., 0.)).unwrap();
            check(Vector3D::new(-1000., -1., 2.)).unwrap();
        }
    }

    #[test]
    #[cfg(feature = "parallel")]
    fn test_par_sampler(){
        let sampler = HorizontalDiskUniformSampler::new(1., 123);
        let _ = sampler.par_map(|(x,y)|{
            println!("x={}, y={}",x,y);
        }).collect::<()>();
        
    }   
    // #[test]
    // fn test_approx_sin(){
    //     const MAX_ERR : Float = 0.0105;
    //     let mut x = 0.0;
    //     loop {
    //         if x > 2.*PI {
    //             break;                
    //         }
    //         let exp = x.sin();
    //         let found = approx_sin(x);
    //         let diff = exp - found;
    //         x += PI/180.;
    //         println!("[SMALL_ANGLE = {}] sin = {} | approx_sin = {} | err = {}", PI/5., exp, found, diff);
    //         if exp >= 0. {
    //             assert!(diff >= 0.);
    //         }else{
    //             assert!(diff <= 0.);
    //         }
    //         assert!( diff.abs() < MAX_ERR);
    //     }
    // }

    // #[test]
    // fn test_approx_cos(){
    //     const MAX_ERR : Float = 0.0023;
    //     let mut x = 0.0;
    //     loop {
    //         if x > 2.*PI {
    //             break;                
    //         }
    //         let exp = x.cos();
    //         let found = approx_cos(x);
    //         let diff = exp - found;
    //         x += PI/180.;
    //         println!("[SMALL_ANGLE = {}] cos = {} | approx_cos = {} | err = {}", PI/5., exp, found, diff);
    //         if exp >= -1e-5 {
    //             assert!(diff >= 0.);
    //         }else{
    //             assert!(diff <= 0.);
    //         }
    //         assert!( diff.abs() < MAX_ERR);
    //     }
    // }
}

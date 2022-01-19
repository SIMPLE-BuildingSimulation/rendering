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

// use rand::prelude::*;

use crate::rand::*;

use crate::{Float,PI};
use crate::samplers::*;
// use geometry3d::intersect_trait::Intersect;
use geometry3d::{Point3D, Triangle3D, Vector3D, Sphere3D, DistantSource3D, Ray3D};
use geometry3d::intersection::IntersectionInfo;


/// Calculates the probability of hitting a 
fn uniform_cone_pdf(cos_theta: Float)->Float{
    let aux = 2.*PI*(1.-cos_theta);
    1./aux
}

/* TRIANGLE */


pub fn sample_triangle_surface(triangle: &Triangle3D, rng: &mut RandGen)->Point3D{
    uniform_sample_triangle(rng, triangle.a(), triangle.b(), triangle.c())
}


pub fn triangle_direction(triangle: &Triangle3D, point: Point3D) -> (Float, Vector3D) {
    const THIRD : Float = 1./3.;
    // Do the rest
    let centroid = (triangle.a() + triangle.b() + triangle.c()) * THIRD;
    let direction = centroid - point;
    let t = direction.length();
    (t, direction / t)
}

pub fn triangle_omega(_triangle: &Triangle3D, _point: Point3D) -> Float {
    unimplemented!();
    // let direction = self.outer_centroid() - point;
    // let t = direction.length_squared();
    // self.area() / t
}

pub fn triangle_solid_angle_pdf(triangle: &Triangle3D, info: &IntersectionInfo, ray: &Ray3D)->Float{
    let d2 = (info.p - ray.origin).length_squared();
    let cos_theta = ray.origin * info.normal;
    debug_assert!(cos_theta > 0.);
    if cos_theta < 1e-7{
        return 0.0
    }
    let area = triangle.area();
    // return
    d2/cos_theta/area
}

/* END OF TRIANGLE */



/* SPHERE */

pub fn sphere_solid_angle_pdf(sphere: &Sphere3D, _info: &IntersectionInfo, ray: &Ray3D)->Float{
    let d2 = (sphere.centre() - ray.origin).length_squared();    
    let sin_theta_2 = sphere.radius * sphere.radius / d2;
    let cos_theta = ( (1. - sin_theta_2).clamp(0., 1.) ).sqrt();    
    // return
    uniform_cone_pdf(cos_theta)
}

pub fn sphere_direction(sphere: &Sphere3D, point: Point3D) -> (Float, Vector3D) {
    let direction = sphere.centre() - point;
    let t = direction.length();
    (t - sphere.radius, direction / t)
}

pub fn sphere_omega(sphere: &Sphere3D, point: Point3D) -> Float {
    let d2 = (sphere.centre() - point).length_squared();       
    PI * sphere.radius * sphere.radius / d2
}

pub fn sample_sphere_surface(sphere:&Sphere3D, rng: &mut RandGen)->Point3D{
    // Sample a sphere of radius 1 centered at the origin
    let p = uniform_sample_sphere(rng);
    let (mut x, mut y, mut z) = (p.x, p.y, p.z);
    // Scale and translate
    x = x.mul_add(sphere.radius, sphere.centre().x);
    y = y.mul_add(sphere.radius, sphere.centre().y);
    z = z.mul_add(sphere.radius, sphere.centre().z);

    // return
    Point3D::new(x, y ,z)
}



/* END SPHERE */



/* DISTANT SOURCE */

pub fn source_solid_angle_pdf(source: &DistantSource3D, _info: &IntersectionInfo, _ray: &Ray3D)->Float{
    
    // return
    uniform_cone_pdf(source.cos_half_alpha)
}

/// It is always in the same direction
pub fn source_direction(source: &DistantSource3D, _point: Point3D) -> (Float, Vector3D) {
    (Float::MAX - 1., source.direction)
}

pub fn source_omega(source: &DistantSource3D, _point: Point3D) -> Float {
    source.omega
}


/* END DISTANT SOURCE */



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_direction() {
        let p = Point3D::new(0., 0., 0.);
        let x = 0.1;
        let y = 10.23;
        let z = 38.1;
        let r = 1.2;
        let sphere = Sphere3D::new(r, Point3D::new(x, y, z));
        let (t, direction) = sphere_direction(&sphere, p);

        assert!((t - (x * x + y * y + z * z).sqrt() + r).abs() < 0.000001);
        assert_eq!(Vector3D::new(x, y, z).get_normalized(), direction);
    }
}

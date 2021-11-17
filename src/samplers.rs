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


use rand::prelude::*;
use crate::{Float,PI};
use geometry3d::{Point3D, Vector3D};

pub fn uniform_sample_triangle(rng: &mut ThreadRng,a:Point3D,b:Point3D,c:Point3D)->Point3D{
    let rand1 : Float = rng.gen();
    let rand2 : Float = rng.gen();
    let aux = rand1.sqrt();
    let u = 1. - aux;
    let v = rand2*aux;
    let v1 = b-a;
    let v2 = c-a;
    // return
    a + v1 * u + v2 * v
}

fn uniform_sample_horizontal_disc(rng: &mut ThreadRng, radius: Float) -> (Float, Float) {
    let r: Float = rng.gen();
    let r = radius * r.sqrt();
    let theta: Float = rng.gen();
    let theta = 2. * PI * theta;

    let local_x = r * theta.sin();
    let local_y = r * theta.cos();
    (local_x, local_y)
}

fn local_to_world(
    normal: Vector3D,
    centre: Point3D,
    x_local: Float,
    y_local: Float,
    z_local: Float,
) -> (Float, Float, Float) {
    debug_assert!((1. - normal.length()).abs() < 100. * Float::EPSILON);
    let local_e3 = normal; //.get_normalized();
    let local_e2 = normal.get_perpendicular().unwrap();
    let local_e1 = local_e2.cross(local_e3);

    debug_assert!((local_e1.length() - 1.).abs() < 0.0000001);
    debug_assert!((local_e2.length() - 1.).abs() < 0.0000001);
    debug_assert!((local_e3.length() - 1.).abs() < 0.0000001);

    let ret = centre + local_e1 * x_local + local_e2 * y_local + local_e3 * z_local;

    (ret.x, ret.y, ret.z)
}

pub fn cosine_weighted_sample_hemisphere(rng: &mut ThreadRng, normal: Vector3D) -> Vector3D {
    let (local_x, local_y) = uniform_sample_horizontal_disc(rng, 1.);
    let local_z = (1. - local_x * local_x - local_y * local_y).sqrt();
    let (x, y, z) = local_to_world(normal, Point3D::new(0., 0., 0.), local_x, local_y, local_z);
    debug_assert!((Vector3D::new(x, y, z).length() - 1.).abs() < 0.0000001);
    Vector3D::new(x, y, z)
}

pub fn uniform_sample_hemisphere(rng: &mut ThreadRng, normal: Vector3D) -> Vector3D {
    // Calculate in
    let rand1: Float = rng.gen();
    let rand2: Float = rng.gen();
    let sq: Float = (1.0 - rand1 * rand1).sqrt();
    let pie2: Float = 2.0 * PI * rand2;
    let local_x = pie2.cos() * sq;
    let local_y = pie2.sin() * sq;
    let local_z = rand1;

    // Take back to world normal
    let (x, y, z) = local_to_world(normal, Point3D::new(0., 0., 0.), local_x, local_y, local_z);
    debug_assert!((Vector3D::new(x, y, z).length() - 1.).abs() < 0.0000001);
    Vector3D::new(x, y, z)
}

pub fn uniform_sample_disc(
    rng: &mut ThreadRng,
    radius: Float,
    centre: Point3D,
    normal: Vector3D,
) -> Point3D {
    let (x_local, y_local) = uniform_sample_horizontal_disc(rng, radius);

    // Form the basis
    let (x, y, z) = local_to_world(normal, centre, x_local, y_local, 0.);
    Point3D::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uniform_sample_disc() {
        fn check(radius: Float, centre: Point3D, normal: Vector3D) -> Result<(), String> {
            let mut rng = rand::thread_rng();
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
            let mut rng = rand::thread_rng();
            let dir = uniform_sample_hemisphere(&mut rng, normal);

            if (1. - dir.length()).abs() > 100. * Float::EPSILON {
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
}

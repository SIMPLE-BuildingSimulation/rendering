use rand::prelude::*;

use geometry3d::vector3d::Vector3D;
use geometry3d::point3d::Point3D;

pub fn sample_disc(rng: &mut ThreadRng, radius: f64, centre: Point3D, normal: Vector3D)->Point3D {
    let r: f64 = rng.gen();
    let r =  radius * r.sqrt();
    let theta :f64 = rng.gen();
    let theta = 2.*std::f64::consts::PI * theta;
    
    let local_x = r * theta.sin();
    let local_y = r * theta.cos();
    
    // Form the basis
    let local_e3 = normal.get_normalized();
    let local_e2 = normal.get_perpendicular().unwrap();
    let local_e1 = local_e2.cross(local_e3);
    debug_assert!(( local_e1.length() - 1. ).abs() < 0.0000001);
    debug_assert!(( local_e2.length() - 1. ).abs() < 0.0000001);
    debug_assert!(( local_e3.length() - 1. ).abs() < 0.0000001);

    // Return
    centre + local_e1 * local_x + local_e2 * local_y
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_sample_disc(){

        fn check(radius: f64, centre: Point3D, normal:Vector3D)->Result<(),String>{
            let mut rng = rand::thread_rng();
            let p = sample_disc(&mut rng, radius, centre, normal);
            if ((p-centre)*normal).abs() > 100.*f64::EPSILON {
                return Err(format!("Point is not coplanar with circle. ((p-centre)*normal).abs() == {}", ((p-centre)*normal).abs()))
            }
            if (p - centre).length() > radius {
                return Err(format!("Sample out of circle. Point sampled was {} | p-centre = {} | radius = {}",p, (p-centre).length(), radius))
            }

            Ok(())
        }

        for _ in 0..100{
            check(1.2, Point3D::new(0., 0., 0.), Vector3D::new(0., 0., 1.)).unwrap();
            check(4.2, Point3D::new(3., 0., 0.), Vector3D::new(0., 1., 1.)).unwrap();
            check(0.12, Point3D::new(0., 1., 0.), Vector3D::new(0., 1., 0.)).unwrap();
            check(23., Point3D::new(0., -10., -20.), Vector3D::new(1., 1., 0.)).unwrap();
            check(23., Point3D::new(0., -10., -20.), Vector3D::new(1., 0., 0.)).unwrap();
        }
    }
}
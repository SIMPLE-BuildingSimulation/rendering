use rand::prelude::*;

use geometry3d::intersect_trait::Intersect;
use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;
use geometry3d::polygon3d::Polygon3D;
use crate::samplers::*;

use geometry3d::plane3d::Plane3D;
use geometry3d::sphere3d::Sphere3D;
use geometry3d::ray3d::Ray3D;

pub trait Sampleable: Intersect {
    /// Receives a [`Point3D`] and returns the distance `t`
    /// and a NORMALIZEd [`Vector3D`] pointing towards it
    fn direction(&self, point: Point3D) -> (f64, Vector3D);

    fn direction_sampler(&self, point:Point3D, n_samples: usize)->Box<dyn Iterator<Item = Vector3D>>;

    /// Returns the solid angle covered by a primitive
    /// as seen—unobstructed—from a certain [`Point3D`].
    fn omega(&self, point: Point3D) -> f64;

    // Samples the center of the
    // fn sample_center(&self)->Spectrum;
}

impl Sampleable for Polygon3D{
    fn direction(&self, point: Point3D) -> (f64, Vector3D) {        
        // Do the rest
        let direction = self.outer_centroid() - point;
        let t = direction.length();
        (t, direction / t)
    }

    fn omega(&self, point: Point3D) -> f64 {
        let direction = self.outer_centroid() - point;
        let t = direction.length();
        self.area()/t/t
    }

    fn direction_sampler(&self, _point:Point3D, _n_samples: usize)->Box<dyn Iterator<Item = Vector3D>>{
        unimplemented!();
    }
}



pub struct SphereSampler{
    pub normal: Vector3D,
    pub ray_origin: Point3D,
    pub centre: Point3D,
    pub radius: f64,
    pub n_samples:usize,
    pub i:usize,
    pub rng : ThreadRng
}

impl Iterator for SphereSampler{
    type Item = Vector3D;
    fn next(&mut self)->Option<Self::Item>{
        if self.n_samples == self.i{
            return None
        }
        self.i+=1;
        let direction = self.centre - self.ray_origin;        
        if self.i == 0 {
            // First sample is center            
            return Some(direction.get_normalized());            
        }
        let p = sample_disc(&mut self.rng, self.radius, self.centre, self.normal);
        let direction = p - self.ray_origin;        
        Some(direction.get_normalized())                
    }

}

impl Sampleable for Sphere3D {
    fn direction(&self, point: Point3D) -> (f64, Vector3D) {
        let direction = self.centre - point;
        let t = direction.length();
        (t - self.radius, direction / t)
    }

    fn omega(&self, point: Point3D) -> f64 {
        let d = (self.centre - point).length();
        let d2 = d * d;
        std::f64::consts::PI * self.r_squared / d2
    }

    fn direction_sampler(&self, point:Point3D, n_samples: usize)->Box<dyn Iterator<Item = Vector3D>>{
        Box::new(SphereSampler{
            n_samples,
            normal: self.centre - point,
            ray_origin:point,
            radius:self.radius,
            centre:self.centre,
            i:0,
            rng: rand::thread_rng()
        })
    }
}

impl Sampleable for Plane3D {
    fn direction(&self, point: Point3D) -> (f64, Vector3D) {
        let centre = if self.d.abs() < f64::EPSILON {
            Point3D::new(0., 0., 0.)
        } else if self.normal.z().abs() > f64::EPSILON {
            Point3D::new(0., 0., self.d / self.normal.z())
        } else if self.normal.y().abs() > f64::EPSILON {
            Point3D::new(0., self.d / self.normal.y(), 0.)
        } else if self.normal.x().abs() > f64::EPSILON {
            Point3D::new(self.d / self.normal.x(), 0., 0.)
        } else {
            unreachable!();
        };

        let direction = centre - point;
        let t = direction.length();
        (t, direction / t)
    }

    fn omega(&self, _: Point3D) -> f64 {
        // planes are infinite... they always light the same
        2. * std::f64::consts::PI
    }

    fn direction_sampler(&self, _point:Point3D, _n_samples: usize)->Box<dyn Iterator<Item = Vector3D>>{
        unimplemented!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_direction() {
        let p = Point3D::new(0., 0., 0.);
        let x = 0.1;
        let y = 10.23;
        let z = 38.1;
        let sphere = Sphere3D::new(1.2, Point3D::new(x, y, z));
        let (t, direction) = sphere.direction(p);

        assert!((t - (x * x + y * y + z * z).sqrt()).abs() < 0.000001);
        assert_eq!(Vector3D::new(x, y, z), direction);
    }
}

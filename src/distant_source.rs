use rand::prelude::*;

use crate::samplers::*;
use crate::sampleable_trait::Sampleable;

use geometry3d::intersect_trait::{Intersect, SurfaceSide};
use geometry3d::point3d::Point3D;
use geometry3d::ray3d::Ray3D;
use geometry3d::vector3d::Vector3D;

pub struct DistantSource3D {
    /// The direction pointing to the source
    direction: Vector3D,
    
    /// The solid angle in Sr
    omega: f64,

    /// The angle in Radians
    angle: f64
}

impl DistantSource3D {
    /// Creates a new [`DistantSource3D`] geometry.
    ///
    /// # Inputs:
    /// * direction: A [`Vector3D`] pointing to the source
    /// * angle: The flat angle in Radians (e.g., a hemisphere would be $`\pi`$)
    pub fn new(direction: Vector3D, angle: f64) -> Self {
        let tan_half_angle = (angle / 2.0).tan();
        let omega = tan_half_angle * tan_half_angle * std::f64::consts::PI;
        let mut direction = direction;
        direction.normalize();
        Self {
            direction,            
            omega,
            angle
        }
    }
}

impl Intersect for DistantSource3D {
    fn intersect(&self, ray: &Ray3D) -> Option<(f64, Vector3D, SurfaceSide)> {
        // it always intersects... just very far away
        Some((f64::MAX - 1., ray.direction * -1., SurfaceSide::Front))
    }
}

pub struct DistantSourceSampler{
    pub normal: Vector3D,
    pub ray_origin: Point3D,
    pub centre: Point3D,
    pub radius: f64,
    pub n_samples:usize,
    pub i:usize,
    pub rng : ThreadRng
}

impl Iterator for DistantSourceSampler{
    type Item = Vector3D;
    fn next(&mut self)->Option<Self::Item>{        
        if self.n_samples == self.i{
            return None
        }
        self.i+=1;
        let direction = self.centre - self.ray_origin;        
        if self.i == 0 {
            // First sample is center            
            return Some(direction.get_normalized())            
        }

        let p = sample_disc(&mut self.rng, self.radius, self.centre, self.normal);
        let direction = p - self.ray_origin;    
        
        Some(direction.get_normalized())
    }

}

impl Sampleable for DistantSource3D {
    /// It is always in the same direction
    fn direction(&self, _point: Point3D) -> (f64, Vector3D) {
        (f64::MAX - 1., self.direction)
    }

    fn omega(&self, _point: Point3D) -> f64 {
        self.omega
    }

    fn direction_sampler(&self, point:Point3D, n_samples: usize)->Box<dyn Iterator<Item = Vector3D>>{
        let normal = self.direction.get_normalized();
        let radius = (self.angle/2.0).tan();
        Box::new(DistantSourceSampler{
            n_samples,
            normal,
            radius,
            ray_origin: point,
            centre: point + normal,
            i:0,
            rng: rand::thread_rng()
        })
    }
}

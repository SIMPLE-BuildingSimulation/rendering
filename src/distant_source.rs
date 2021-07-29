use rand::prelude::*;

use crate::sampleable_trait::Sampleable;
use crate::samplers::*;

use geometry3d::distant_source3d::DistantSource3D;
use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

pub struct DistantSourceSampler {
    pub normal: Vector3D,
    pub ray_origin: Point3D,
    pub centre: Point3D,
    pub radius: f64,
    pub n_samples: usize,
    pub i: usize,
    pub rng: ThreadRng,
}

impl Iterator for DistantSourceSampler {
    type Item = Vector3D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_samples == self.i {
            return None;
        }
        self.i += 1;
        let direction = self.centre - self.ray_origin;
        if self.i == 0 {
            // First sample is center
            return Some(direction.get_normalized());
        }

        let p = uniform_sample_disc(&mut self.rng, self.radius, self.centre, self.normal);
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

    fn direction_sampler(
        &self,
        point: Point3D,
        n_samples: usize,
    ) -> Box<dyn Iterator<Item = Vector3D>> {
        let normal = self.direction.get_normalized();
        let radius = (self.angle / 2.0).tan();
        Box::new(DistantSourceSampler {
            n_samples,
            normal,
            radius,
            ray_origin: point,
            centre: point + normal,
            i: 0,
            rng: rand::thread_rng(),
        })
    }
}

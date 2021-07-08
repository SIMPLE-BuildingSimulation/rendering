use crate::sampleable_trait::Sampleable;
use geometry3d::intersect_trait::{Intersect, SurfaceSide};
use geometry3d::point3d::Point3D;
use geometry3d::ray3d::Ray3D;
use geometry3d::vector3d::Vector3D;

pub struct DistantSource3D {
    /// The direction pointing to the source
    direction: Vector3D,

    /// The planar angle of the source (i.e., as drown in 2D) in radians
    angle: f64,
}

impl DistantSource3D{
    pub fn new(direction:Vector3D, angle:f64)->Self{
        let mut direction = direction;
        direction.normalize();
        Self{
            direction,
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

impl Sampleable for DistantSource3D {
    /// It is always in the same direction
    fn direction(&self, _point: Point3D) -> (f64, Vector3D) {
        (f64::MAX - 1., self.direction)
    }
}

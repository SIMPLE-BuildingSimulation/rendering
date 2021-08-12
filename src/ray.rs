use crate::Float;
use geometry3d::transform::Transform;
use geometry3d::ray3d::Ray3D;
use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

pub struct Ray{
    pub geometry: Ray3D,
    pub time: Float,
    // medium.
}

impl Ray{

    pub fn apply_transformation(&self, t:&Transform)->Self{
        let (geometry,_o_error, _d_error)= t.transform_ray(&self.geometry);
        Self{
            geometry,
            time: self.time,
            // medium: self.medium
        }
    }

    #[inline(always)]
    pub fn direction(&self)->Vector3D{
        self.geometry.direction
    }

    #[inline(always)]
    pub fn origin(&self)->Point3D{
        self.geometry.origin
    }
}


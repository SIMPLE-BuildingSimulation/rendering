use geometry3d::intersect_trait::Intersect;
use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;

use geometry3d::sphere3d::Sphere3D;
use geometry3d::plane3d::Plane3D;

pub trait Sampleable : Intersect {
    /// Receives a [`Point3D`] and returns the distance `t`
    /// and a NORMALIZEd [`Vector3D`] pointing towards it
    fn direction(&self, point: Point3D) -> (f64, Vector3D);
}

impl Sampleable for Sphere3D {
    fn direction(&self, point: Point3D) -> (f64, Vector3D) {
        let direction = self.centre - point;
        let t = direction.length();
        (t-self.radius, direction / t)
    }
}

impl Sampleable for Plane3D {
    fn direction(&self, point: Point3D) -> (f64, Vector3D) {
        
        let centre = if self.d.abs()<f64::EPSILON{
            Point3D::new(0.,0.,0.)
        }else if self.normal.z().abs() > f64::EPSILON{
            Point3D::new(0., 0., self.d/self.normal.z())
        }else if self.normal.y().abs()>f64::EPSILON{
            Point3D::new(0., self.d/self.normal.y(), 0.)
        }else if self.normal.x().abs()>f64::EPSILON{
            Point3D::new(self.d/self.normal.x(),0., 0.)
        }else{
            unreachable!();
        };

        let direction = centre - point;
        let t = direction.length();
        (t, direction / t)
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

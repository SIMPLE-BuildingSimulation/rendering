use geometry3d::point3d::Point3D;
use geometry3d::ray3d::Ray3D;
use geometry3d::vector3d::Vector3D;

pub struct View {
    pub view_point: Point3D,
    pub view_direction: Vector3D,
    pub view_up: Vector3D,

    /// Horizontal angle of the Field of View (i.e., frustum) in degrees
    pub field_of_view: f64,

    /// Width of the image, in pixels
    pub width: usize,

    /// Width/height
    pub aspect_ratio: f64,
    // u,v,w? v=view_up; w=view_direction; u=w x v
}

impl View {
    /// Returns the (width, height) values
    pub fn dimensions(&self) -> (usize, usize) {
        let height = (self.aspect_ratio * self.width as f64).round() as usize;
        return (self.width, height);
    }
}

impl Default for View {
    fn default() -> Self {
        Self {
            view_point: Point3D::new(0., 0., 0.),
            view_direction: Vector3D::new(0., 1., 0.),
            view_up: Vector3D::new(0., 0., 1.),
            field_of_view: 60.,
            aspect_ratio: 4. / 3.,
            width: 256,
        }
    }
}

pub enum Camera {
    Pinhole,
}

fn pinhole_primary_rays(view: &View) -> Vec<Ray3D> {
    let mut view_direction = view.view_direction;
    view_direction.normalize();
    let mut view_up = view.view_up;
    view_up.normalize();

    let (width, height) = view.dimensions();
    let mut rays = Vec::with_capacity(height * width);

    let u = view.view_direction.cross(view.view_up);
    let distance_to_screen = 1. / (std::f64::consts::PI * view.field_of_view / 180.0 / 2.0).tan();

    // Calcuate the step
    let dx = 2. / width as f64;
    let dy = 2. / height as f64;
    // Iterate all pixels
    let mut y = 1. - dy / 2.;
    while y >= -1. {
        let mut x = -1. + dx / 2.;
        while x <= 1. {
            // Calculate direction
            let mut direction = view.view_direction * distance_to_screen + u * x + view.view_up * y;

            direction.normalize();

            // push
            rays.push(Ray3D {
                origin: view.view_point,
                direction,
            });

            // Move right
            x += dx;
        }
        // Move down
        y -= dy;
    }

    rays
}

impl Camera {
    pub fn get_primary_rays(&self, view: &View) -> Vec<Ray3D> {
        match self {
            Self::Pinhole => pinhole_primary_rays(view),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinhole_primary_rays() {
        let camera = Camera::Pinhole;
        let width = 100;
        let aspect_ratio = 1.;
        let rays = camera.get_primary_rays(&View {
            width,
            aspect_ratio,
            ..View::default()
        });
        assert_eq!(rays.len(), width * width);
    }
}

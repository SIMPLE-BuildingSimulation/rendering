use crate::film::Film;
use geometry3d::point3d::Point3D;
use geometry3d::ray3d::Ray3D;
use geometry3d::vector3d::Vector3D;

pub struct View {
    pub view_point: Point3D,
    pub view_direction: Vector3D,
    pub view_up: Vector3D,

    /// Horizontal angle of the Field of View (i.e., frustum) in degrees
    pub field_of_view: f64,
}

impl Default for View {
    fn default() -> Self {
        Self {
            view_point: Point3D::new(0., 0., 0.),
            view_direction: Vector3D::new(0., 1., 0.),
            view_up: Vector3D::new(0., 0., 1.),
            field_of_view: 60.,
        }
    }
}

/// Used for getting a sample ray from the [`Camera`]
pub struct CameraSample {
    /// The position (x,y) within the [`Film`]
    pub p_film: (usize, usize),

    /// The position within the Lens of the camera
    pub p_lens: (f64, f64),

    /// Time at which the ray will be emmited
    pub time: f64,
}

pub trait Camera {
    /// Generates a ray and
    fn gen_ray(&self, sample: &CameraSample) -> (Ray3D, f64);

    /// Gets the film resolution (width,height) in pixels
    fn film_resolution(&self) -> (usize, usize);

    /// Borrows the view
    fn view(&self) -> &View;
}

pub struct PinholeCam {
    view: View,
    film: Film,
    film_distance: f64,

    /// A [`Vector3D`] which is the result of view_direction.cross(view_up)
    u: Vector3D,
}

impl PinholeCam {
    pub fn new(view: View, film: Film) -> Self {
        let film_distance = 1. / (std::f64::consts::PI * view.field_of_view / 180.0 / 2.0).tan();
        let u = view.view_direction.cross(view.view_up);
        Self {
            view,
            film,
            film_distance,
            u,
        }
    }
}

impl Camera for PinholeCam {
    fn gen_ray(&self, sample: &CameraSample) -> (Ray3D, f64) {
        let (width, height) = self.film.resolution;
        let (x_pixel, y_pixel) = sample.p_film;
        let dx = 2. / width as f64;
        let dy = 2. / height as f64;

        let x = dx / 2. + x_pixel as f64 * dx - 1.;
        let y = dy / 2. + y_pixel as f64 * dy - 1.;

        let direction =
            self.view.view_direction * self.film_distance + self.u * x - self.view.view_up * y;

        let ray = Ray3D {
            direction: direction.get_normalized(),
            origin: self.view.view_point,
        };

        // return
        (ray, 1.)
    }

    fn film_resolution(&self) -> (usize, usize) {
        self.film.resolution
    }

    fn view(&self) -> &View {
        &self.view
    }
} // end of PinholeCam

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn test_pinhole_primary_rays() {
    //     let camera = Camera::Pinhole;
    //     let width = 100;
    //     let aspect_ratio = 1.;
    //     let rays = camera.get_primary_rays(&View {
    //         width,
    //         aspect_ratio,
    //         ..View::default()
    //     });
    //     assert_eq!(rays.len(), width * width);
    // }
}

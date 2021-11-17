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

use crate::Float;
use crate::film::Film;
use geometry3d::{Point3D, Ray3D, Vector3D};
use crate::ray::Ray;

pub struct View {
    pub view_point: Point3D,
    pub view_direction: Vector3D,
    pub view_up: Vector3D,

    /// Horizontal angle of the Field of View (i.e., frustum) in degrees
    pub field_of_view: Float,
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
    pub p_lens: (Float, Float),

    /// Time at which the ray will be emmited
    pub time: Float,
}

pub trait Camera {
    /// Generates a ray and
    fn gen_ray(&self, sample: &CameraSample) -> (Ray, Float);

    /// Gets the film resolution (width,height) in pixels
    fn film_resolution(&self) -> (usize, usize);

    /// Borrows the view
    fn view(&self) -> &View;
}

pub struct PinholeCam {
    view: View,
    film: Film,
    film_distance: Float,

    /// A [`Vector3D`] which is the result of view_direction.cross(view_up)
    u: Vector3D,
}

impl PinholeCam {
    pub fn new(view: View, film: Film) -> Self {
        let film_distance = 1. / (view.field_of_view.to_radians() / 2.0).tan();
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
    fn gen_ray(&self, sample: &CameraSample) -> (Ray, Float) {
        let (width, height) = self.film.resolution;
        let (x_pixel, y_pixel) = sample.p_film;
        let dx = 2. / width as Float;
        let dy = 2. / height as Float;

        let x = dx / 2. + x_pixel as Float * dx - 1.;
        let y = dy / 2. + y_pixel as Float * dy - 1.;

        let direction =
            self.view.view_direction * self.film_distance + self.u * x - self.view.view_up * y;

        let ray = Ray{
            geometry: Ray3D {
                direction: direction.get_normalized(),
                origin: self.view.view_point,
            }, 
            time: sample.time,
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

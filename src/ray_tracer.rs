use crate::camera::{Camera, View};
use crate::colour::Spectrum;
use crate::image::ImageBuffer;
use crate::scene::Scene;
use geometry3d::ray3d::Ray3D;
use geometry3d::vector3d::Vector3D;

pub struct RayTracer {}

impl RayTracer {
    pub fn render(scene: &Scene, camera: &Camera, view: &View) -> ImageBuffer {
        let primary_rays = camera.get_primary_rays(view);
        let (width, height) = view.dimensions();
        let mut buffer = ImageBuffer::new(width, height);

        for ray in primary_rays.iter() {
            if let Some((t, normal, material_index)) = scene.cast_ray(ray) {
                debug_assert!((1.0 - normal.length()).abs() < f64::EPSILON);
                // Normalize normal?

                // shade.
                //for (i,light) in scene.lights().iter().enumerate(){

                //}
                let mut light_direction = Vector3D::new(0., 0., 1.);
                light_direction.normalize();
                let light_size = 0.5; // degrees
                let light_size = light_size * std::f64::consts::PI / 180. / 2.;
                let omega = light_size.tan() * light_size.tan() * std::f64::consts::PI;

                let light_r = 100.;
                let light_g = 100.;
                let light_b = 100.;

                let shadow_ray = Ray3D {
                    origin: ray.project(t) + normal * 0.0001,
                    direction: light_direction,
                };
                // shadow_ray.advance();// Prevent self-shadow
                if let None = scene.cast_ray(&shadow_ray) {
                    let cos_theta = (normal * light_direction).abs();

                    buffer.push(Spectrum {
                        red: light_r
                            * omega
                            * cos_theta
                            * scene.borrow_material(material_index).red()
                            / std::f64::consts::PI,
                        green: light_g
                            * omega
                            * cos_theta
                            * scene.borrow_material(material_index).green()
                            / std::f64::consts::PI,
                        blue: light_b
                            * omega
                            * cos_theta
                            * scene.borrow_material(material_index).blue()
                            / std::f64::consts::PI,
                    })
                } else {
                    // It does not see the light
                    buffer.push(Spectrum {
                        red: 0.,
                        green: 0.,
                        blue: 0.,
                    });
                }
            } else {
                // Did not hit.
                buffer.push(Spectrum {
                    red: 0.,
                    green: 0.,
                    blue: 0.,
                });
            }
        }
        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Plastic;
    use geometry3d::plane3d::Plane3D;
    use geometry3d::point3d::Point3D;
    use geometry3d::sphere3d::Sphere3D;

    #[test]
    fn test_raycast() {
        // Build scene
        let mut scene = Scene::default();
        let front_mat_index = scene.push_material(Box::new(Plastic {
            red: 0.3,
            green: 0.05,
            blue: 0.05,
            specularity: 0.,
            roughness: 0.,
        }));

        let back_mat_index = scene.push_material(Box::new(Plastic {
            red: 0.05,
            green: 0.3,
            blue: 0.05,
            specularity: 0.,
            roughness: 0.,
        }));

        scene.push_object(
            front_mat_index,
            back_mat_index,
            Box::new(Sphere3D::new(1.5, Point3D::new(0., 0., 0.5))),
        );

        scene.push_object(
            back_mat_index,
            front_mat_index,
            Box::new(Sphere3D::new(1.5, Point3D::new(1., -1., -1.5))),
        );

        scene.push_object(
            back_mat_index,
            back_mat_index,
            Box::new(Plane3D::new(
                Point3D::new(0., 0., -3.),
                Vector3D::new(0., 0., 1.),
            )),
        );
        // Create camera
        let camera = Camera::Pinhole;

        // Create view
        let width = 512;
        let aspect_ratio = 1.;
        let view = View {
            width,
            aspect_ratio,
            view_direction: Vector3D::new(0., 1., 0.),
            view_point: Point3D::new(0., -13., 0.),
            ..View::default()
        };

        let buffer = RayTracer::render(&scene, &camera, &view);
        buffer.save_jpeg("./test_imgs/ray_tracer.jpeg".to_string());
        buffer.save_hdre("./test_imgs/ray_tracer.hdr".to_string());
    }
}

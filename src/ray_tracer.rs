use crate::camera::{Camera, CameraSample};
// use crate::colour::Spectrum;
use crate::image::ImageBuffer;
use crate::scene::Scene;

pub struct RayTracer {}

impl RayTracer {
    pub fn render(scene: &Scene, camera: &dyn Camera) -> ImageBuffer {
        let (width, height) = camera.film_resolution();
        let mut buffer = ImageBuffer::new(width, height);
        let total_pixels = width * height;

        let mut last_progress: f64 = 0.0;
        let mut i = 0;
        for y in 0..height {
            for x in 0..width {
                let (ray, _weight) = camera.gen_ray(&CameraSample {
                    p_film: (x, y),
                    p_lens: (0., 0.), // we will not sue this
                    time: 1.,         // we will not use
                });
                buffer[(x, y)] = scene.trace_ray(&ray, 0);
                // report
                let progress = (100 * i) as f64 / total_pixels as f64;
                if (progress - progress.floor()) < 0.1 && (progress - last_progress).abs() > 1. {
                    last_progress = progress;
                    println!("... Done {:.0}%", progress);
                }
                // increase counter
                i += 1;
            }
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::process::Command;

    // use geometry3d::ray3d::Ray3D;
    use geometry3d::vector3d::Vector3D;

    use geometry3d::point3d::Point3D;

    use crate::camera::{PinholeCam, View};
    use crate::film::Film;
    use std::time::Instant;

    fn compare_with_radiance(filename: String) {
        let now = Instant::now();

        let scene = Scene::from_radiance(format!("./test_data/{}", filename));

        // Create film
        let film = Film {
            resolution: (512, 512),
        };

        // Create view
        let view = View {
            view_direction: Vector3D::new(0., 1., 0.),
            view_point: Point3D::new(0., -13., 0.),
            ..View::default()
        };

        // Create camera
        let camera = PinholeCam::new(view, film);

        let buffer = RayTracer::render(&scene, &camera);

        println!(
            "Scene '{}' took {} seconds to render",
            filename,
            now.elapsed().as_secs()
        );

        buffer.save_hdre(format!("./test_data/images/self_{}.hdr", filename));
    }

    #[test]
    fn render_scenes() {
        compare_with_radiance("exterior_0_diffuse_plastic.rad".to_string());
        // compare_with_radiance("exterior_0_specularity.rad".to_string());
        compare_with_radiance("exterior_0_mirror.rad".to_string());
        // compare_with_radiance("exterior_0_dielectric.rad".to_string());
    }

    use crate::material::{Light, Mirror, Plastic};
    use geometry3d::distant_source3d::DistantSource3D;
    use geometry3d::plane3d::Plane3D;
    use geometry3d::sphere3d::Sphere3D;

    #[test]
    fn test_2() {
        // Build scene
        let mut scene = Scene::default();

        let red = scene.push_material(Box::new(Plastic {
            red: 0.55,
            green: 0.55,
            blue: 0.55,
            specularity: 0.,
            roughness: 0.,
        }));

        let green = scene.push_material(Box::new(Plastic {
            red: 0.15,
            green: 0.15,
            blue: 0.15,
            specularity: 0.,
            roughness: 0.,
        }));

        let mirror = scene.push_material(Box::new(Mirror {
            red: 0.8,
            green: 0.99,
            blue: 0.8,
        }));

        scene.push_object(
            mirror,
            mirror,
            Box::new(Sphere3D::new(1.5, Point3D::new(0., 0., 0.5))),
        );

        scene.push_object(
            red,
            red,
            Box::new(Sphere3D::new(1.5, Point3D::new(1., -1., -1.5))),
        );

        scene.push_object(
            green,
            green,
            Box::new(Plane3D::new(
                Point3D::new(0., 0., -3.),
                Vector3D::new(0., 0., 1.),
            )),
        );

        let up = scene.push_material(Box::new(Light {
            red: 10000.,
            green: 10000.,
            blue: 10000.,
        }));

        scene.push_object(
            up,
            up,
            Box::new(DistantSource3D::new(
                Vector3D::new(0., 0., 1.),         // direction
                0.5 * std::f64::consts::PI / 180., // angle
            )),
        );

        scene.push_object(
            up,
            up,
            Box::new(DistantSource3D::new(
                Vector3D::new(0., -1., 1.),        // direction
                0.5 * std::f64::consts::PI / 180., // angle
            )),
        );

        let lightbulb = scene.push_material(Box::new(Light {
            red: 10.,
            green: 10.,
            blue: 10.,
        }));

        scene.push_object(
            lightbulb,
            lightbulb,
            Box::new(Sphere3D::new(1.5, Point3D::new(1., -1., 15.))),
        );

        // Create camera
        // Create film
        let film = Film {
            resolution: (512, 512),
        };

        // Create view
        let view = View {
            view_direction: Vector3D::new(0., 1., 0.),
            view_point: Point3D::new(0., -13., 0.),
            ..View::default()
        };

        // Create camera
        let camera = PinholeCam::new(view, film);

        let buffer = RayTracer::render(&scene, &camera);

        buffer.save_hdre("./test_data/images/ray_caster.hdr".to_string());
    }
}

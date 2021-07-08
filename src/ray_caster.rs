use crate::camera::{Camera, View};
use crate::colour::Spectrum;
use crate::image::ImageBuffer;
use crate::scene::Scene;

pub struct RayCaster {}

impl RayCaster {
    pub fn render(scene: &Scene, camera: &Camera, view: &View) -> ImageBuffer {
        let primary_rays = camera.get_primary_rays(view);
        let (width, height) = view.dimensions();
        let mut buffer = ImageBuffer::new(width, height);

        for ray in primary_rays.iter() {
            // If hits an object
            if let Some((t, normal, material_index)) = scene.cast_ray(ray) {
                debug_assert!((1.0 - normal.length()).abs() < f64::EPSILON);

                let material = scene.borrow_material(material_index);

                buffer.push(scene.get_local_illumination(material, ray.direction, ray.project(t), normal));
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
    // use geometry3d::ray3d::Ray3D;
    use geometry3d::vector3d::Vector3D;

    use crate::material::Plastic;
    use crate::material::Light;
    use crate::distant_source::DistantSource3D;
    use geometry3d::plane3d::Plane3D;
    use geometry3d::point3d::Point3D;
    use geometry3d::sphere3d::Sphere3D;

    #[test]
    fn test_raycast() {
        // Build scene
        let mut scene = Scene::default();
        let red = scene.push_material(Box::new(Plastic {
            red: 0.3,
            green: 0.05,
            blue: 0.05,
            specularity: 0.,
            roughness: 0.,
        }));

        let green = scene.push_material(Box::new(Plastic {
            red: 0.05,
            green: 0.3,
            blue: 0.05,
            specularity: 0.,
            roughness: 0.,
        }));

        let light = scene.push_material(Box::new(Light {
            red: 1000.,
            green: 100.,
            blue: 100.,            
        }));

        scene.push_object(
            red,
            green,
            Box::new(Sphere3D::new(1.5, Point3D::new(0., 0., 0.5))),
        );

        scene.push_object(
            green,
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

        scene.push_object(
            light,
            light,
            Box::new(DistantSource3D::new(
                Vector3D::new(0., 0., 1.),
                0.5 * std::f64::consts::PI/180.,
            ))
        );

        scene.push_object(
            light,
            light,
            Box::new(DistantSource3D::new(
                Vector3D::new(0., 1., 1.),
                0.5 * std::f64::consts::PI/180.,
            ))
        );

        // scene.push_object(
        //     light,
        //     light,
        //     Box::new(Sphere3D::new(
        //         1.5,
        //         Point3D::new(1., -1., 15.)
        //     ))
        // );



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

        let buffer = RayCaster::render(&scene, &camera, &view);
        buffer.save_jpeg("./test_imgs/ray_caster.jpeg".to_string());
        buffer.save_hdre("./test_imgs/ray_caster.hdr".to_string());
        panic!("asd")
    }
}

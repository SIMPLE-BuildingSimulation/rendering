use crate::camera::{Camera, View};
// use crate::colour::Spectrum;
use crate::image::ImageBuffer;
use crate::scene::Scene;


pub struct RayCaster {}

impl RayCaster {
    pub fn render(scene: &Scene, camera: &Camera, view: &View) -> ImageBuffer {
        let primary_rays = camera.get_primary_rays(view);
        let (width, height) = view.dimensions();
        let mut buffer = ImageBuffer::new(width, height);

        for ray in primary_rays.iter() {
            buffer.push(scene.trace_ray(ray, 0));
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

    fn compare_with_radiance(filename:String){

        let scene = Scene::from_radiance(format!("./test_data/{}", filename));        

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
        buffer.save_hdre(format!("./test_data/images/self_{}.hdr",filename));

        // // Now in Radiance...
        // let output = if cfg!(target_os = "windows") {
        //     Command::new("cmd")
        //             .arg(format!("oconv ./test_data/{} > ./test_data/octree.oct", filename))
        //             .output()
        //             .expect("failed to execute process")
        // } else {
        //     Command::new("sh")
        //             .arg("-c")
        //             .arg(format!("oconv ./test_data/{} > ./test_data/octree.oct ", filename))
        //             .output()
        //             .expect("failed to execute process")
        // };

        // let output = if cfg!(target_os = "windows") {
        //     Command::new("cmd")
        //             .arg(format!("rpict -ab 0 -vp 0 -13 0 -vd 0 1 0 -vh 60 -vv 60 ./test_data/octree.oct > ./test_data/images/radiance_{}.hdr", filename))
        //             .output()
        //             .expect("failed to execute process")
        // } else {
        //     Command::new("sh")
        //             .arg("-c")
        //             .arg(format!("rpict -ab 0 -vp 0 -13 0 -vd 0 1 0 -vh 60 -vv 60 ./test_data/octree.oct > ./test_data/images/radiance_{}.hdr", filename))
        //             .output()
        //             .expect("failed to execute process")
        // };
    }

    #[test]
    fn test_raycast() {
        
        compare_with_radiance("exterior_0_diffuse_plastic.rad".to_string());
        compare_with_radiance("exterior_0_mirror.rad".to_string());
        
    }



    use crate::material::Plastic;
    use geometry3d::plane3d::Plane3D;    
    use geometry3d::sphere3d::Sphere3D;
    use crate::material::Light;
    use crate::distant_source::DistantSource3D;

    #[test]
    fn test_2(){
        // Build scene
        let mut scene = Scene::default();

        let red = scene.push_material(Box::new(Plastic {
            red: 0.3,
            green: 0.05,
            blue: 0.05,
            specularity: 0.,
            roughness: 0.,
        }));

        let green  = scene.push_material(Box::new(Plastic {
            red: 0.05,
            green: 0.3,
            blue: 0.05,
            specularity: 0.,
            roughness: 0.,
        }));

        scene.push_object(
            red,
            red,
            Box::new(Sphere3D::new(1.5, Point3D::new(0., 0., 0.5))),
        );

        scene.push_object(
            green,
            green,
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

        let up = scene.push_material(Box::new(Light{
            red: 10000., green: 1000., blue:1000.
        }));

        scene.push_object(up,up,Box::new(DistantSource3D::new(
            Vector3D::new(0.,0.,1.), // direction
            0.5*std::f64::consts::PI/180. // angle
        )));

        scene.push_object(up,up,Box::new(DistantSource3D::new(
            Vector3D::new(0.,1.,1.), // direction
            0.5*std::f64::consts::PI/180. // angle
        )));

        let lightbulb = scene.push_material(Box::new(Light{
            red: 10., green: 1., blue:1.
        }));

        scene.push_object(
            lightbulb,
            lightbulb,
            Box::new(Sphere3D::new(1.5, Point3D::new(1., -1., 15.))),
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

        let buffer = RayCaster::render(&scene, &camera, &view);        
        buffer.save_hdre("./test_data/images/ray_caster.hdr".to_string());
    }
}

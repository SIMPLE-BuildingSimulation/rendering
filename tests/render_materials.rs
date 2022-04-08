
use rendering::ray_tracer::RayTracer;
use rendering::camera::{Pinhole, View, Film};
use geometry3d::{Point3D};
use rendering::material::Material;


use rendering::scene::Scene;
use rendering::primitive::Primitive;
use rendering::colour::Spectrum;
use rendering::material::*;
use geometry3d::{    
    Sphere3D
};

fn render_ball(mat: Box<dyn Material + Sync>, filename: &str){
 
    let mut scene = Scene::new();

    // Add ball
    let centre = Point3D::new(0., 0., 1.5);
    let mat = scene.push_material(mat);
    let s = Sphere3D::new(
        1.5,
        centre
    );
    scene.push_object(
        mat,
        mat,
        Primitive::Sphere(s),
    );

    // Add light
    let glow = scene.push_material(Box::new(Light(
        //145, 7, 205
        Spectrum {
            red: 1.,
            green: 1.,
            blue: 1.,
        } * 10000.        
    )));

    let s = Sphere3D::new(
        0.1,
        Point3D::new(4., -15., 5.),        
    );
    scene.push_object(
        glow,
        glow,
        Primitive::Sphere(s),
    );

    scene.build_accelerator();

    // Create film
    let film = Film {
        resolution: (512, 512),
    };

    // Create view 
    let view_point = Point3D::new(0., -10., 4.1);
    let view_direction = (centre  -  view_point).get_normalized(); 
    let view = View {
        view_direction,
        view_point,
        ..View::default()
    };

    // Create camera
    let camera = Pinhole::new(view, film);

    let integrator = RayTracer {
        n_ambient_samples: 300,
        n_shadow_samples: 10,
        max_depth: 2,
        ..RayTracer::default()
    };
    
    let buffer = integrator.render(&scene, &camera);    
    buffer.save_hdre(std::path::Path::new(filename));


}

#[test]
// #[ignore]
fn test_render_specular_plastic(){
    // cargo test --features parallel --release --package rendering --test glow_sphere -- render_specular_plastic --exact --nocapture
    
    
    let plastic = Box::new(Plastic{
        colour: Spectrum {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
        },
        specularity: 0.28,
        roughness: 0.05,
    });
    
    
    render_ball(plastic, "./test_data/images/specular_plastic.hdr")
}


#[test]
// #[ignore]
fn test_render_mirror(){
    // cargo test --features parallel --release --package rendering --test glow_sphere -- render_specular_plastic --exact --nocapture
    
    
    let plastic = Box::new(Mirror(Spectrum {
        red: 0.5,
        green: 0.5,
        blue: 0.5,
    }));
    
    
    render_ball(plastic, "./test_data/images/mirror.hdr")
}
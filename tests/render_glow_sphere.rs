use geometry3d::Point3D;
use rendering::camera::{Film, Pinhole, View};
use rendering::ray_tracer::RayTracer;
use rendering::Float;

mod glow_sphere_scene;

#[test]
// #[ignore]
fn test_render_glow_sphere() {
    // cargo test --features parallel --release --package rendering --test glow_sphere -- test_render_glow_sphere --exact --nocapture
    const SCENE_HEIGHT: Float = 40.;
    const GROUND_SIZE: Float = 100000.;
    let mut scene = glow_sphere_scene::get_scene(SCENE_HEIGHT, GROUND_SIZE);

    scene.build_accelerator();

    // Create film
    let film = Film {
        resolution: (512, 512),
    };

    // Create view
    let view_point = Point3D::new(0., -10., 4.1 + SCENE_HEIGHT);
    let view_direction = (Point3D::new(0., 0., 1.5 + SCENE_HEIGHT) - view_point).get_normalized();
    let view = View {
        view_direction,
        view_point,
        ..View::default()
    };

    // Create camera
    let camera = Pinhole::new(view, film);

    let integrator = RayTracer {
        n_ambient_samples: 300,
        n_shadow_samples: 300,
        max_depth: 2,
        ..RayTracer::default()
    };

    let buffer = integrator.render(&scene, &camera);
    buffer.save_hdre(std::path::Path::new("./test_data/images/sphere_glow.hdr"));
}

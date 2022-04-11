use geometry3d::{Point3D, Vector3D};
use rendering::camera::{Film, Pinhole, View};
use rendering::ray_tracer::RayTracer;

use rendering::scene::Scene;

#[test]
#[ignore]
fn test_render_cornell() {
    // 60 seconds
    // cargo test --package rendering --features parallel --release --test test_render_cornell -- test_render_cornell --exact --nocapture --ignored
    // oconv ../room.rad > room.oct ;time rpict -x 512 -y 512 -vv 60 -vh 60 -ab 3 -ad 220 -aa 0 -vp 2 1 1 -vd 0 1 0 ./room.oct > rad_room.hdr

    let mut scene = Scene::from_radiance("./test_data/cornell.rad".to_string());

    scene.build_accelerator();

    // Create camera
    let film = Film {
        resolution: (512, 367),
        // resolution: (1024, 768),
        // resolution: (512, 512),
    };

    // Create view
    let view = View {
        view_direction: Vector3D::new(0., 1., 0.).get_normalized(),
        // view_point: Point3D::new(2., 1., 1.),
        view_point: Point3D::new(3., -5., 2.25),
        field_of_view: 50.,
        ..View::default()
    };

    // Create camera
    let camera = Pinhole::new(view, film);

    let integrator = RayTracer {
        n_ambient_samples: 90,
        n_shadow_samples: 1,
        max_depth: 3,
        ..RayTracer::default()
    };


    let buffer = integrator.render(&scene, &camera);
    buffer.save_hdre(std::path::Path::new("./test_data/images/cornell.hdr"));
}

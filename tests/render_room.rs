use geometry3d::{Point3D, Vector3D};
use rendering::camera::{Film, Pinhole, View};
use rendering::ray_tracer::RayTracer;

use rendering::scene::Scene;

#[test]
#[ignore]
fn test_render_room() {
    // 60 seconds
    // cargo test --release --features parallel --package rendering --test test_render_room -- test_render_room --exact --nocapture --ignored
    // oconv ../room.rad ../white_sky.rad > room.oct ;time rpict -x 512 -y 512 -vv 60 -vh 60 -ab 3 -ad 220 -aa 0 -vp 2 1 1 -vd 0 1 0 ./room.oct > rad_room.hdr

    let mut scene = Scene::from_radiance("./test_data/room.rad".to_string());
    scene.add_perez_sky(
        calendar::Date {
            month: 6,
            day: 1,
            hour: 12.,
        },
        -33.,
        70.,
        65.,
        200.,
        500.,
    );

    scene.build_accelerator();

    // Create film
    let film = Film {
        resolution: (512, 512),
    };

    // Create view
    let view = View {
        view_direction: Vector3D::new(0., 1., 0.).get_normalized(),
        view_point: Point3D::new(2., 1., 1.),
        ..View::default()
    };
    // Create camera
    let camera = Pinhole::new(view, film);

    let integrator = RayTracer {
        n_ambient_samples: 220,
        n_shadow_samples: 1,
        max_depth: 3,
        ..RayTracer::default()
    };

    let buffer = integrator.render(&scene, &camera);
    buffer.save_hdre(std::path::Path::new("./test_data/images/room.hdr"));
}

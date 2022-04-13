use geometry3d::Point3D;
use rendering::camera::{Film, Pinhole, View};
use rendering::ray_tracer::RayTracer;

use geometry3d::{Sphere3D, Triangle3D};
use rendering::colour::Spectrum;
use rendering::material::*;
use rendering::primitive::Primitive;
use rendering::scene::Scene;
use rendering::{Float, PI};

#[test]
// #[ignore]
fn test_render_laptop() {
    // cargo test --features parallel --release --package rendering --test test_render_laptop -- test_render_laptop --exact --nocapture
    let mut scene = Scene::new();

    const BASE_THICKNESS: Float = 0.01;
    const SCREEN_THICKNESS: Float = 0.006;
    const WIDTH: Float = 0.38;
    const DEPTH: Float = 0.3;
    const ANGLE: Float = 30. * PI / 180.0;
    const GROUND_SIZE: Float = 10.;
    const OFFSET: Float = 0.002;

    // Add light
    let glow = scene.push_material(Box::new(Light(
        Spectrum {
            red: 1.,
            green: 1.,
            blue: 1.,
        } * 500.,
    )));

    let s = Sphere3D::new(0.1, Point3D::new(0., 0., 5.));
    scene.push_object(glow, glow, Primitive::Sphere(s));

    // Materials
    let plastic = Box::new(Plastic {
        colour: Spectrum {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
        },
        specularity: 0.05,
        roughness: 0.1,
    });
    let plastic = scene.push_material(plastic);

    let screen = Box::new(Light(
        Spectrum {
            red: 145.,
            green: 7.,
            blue: 205.,
        } * 0.3,
    ));
    let screen = scene.push_material(screen);

    let concrete = Box::new(Plastic {
        colour: Spectrum {
            red: 0.2,
            green: 0.2,
            blue: 0.2,
        },
        specularity: 0.0,
        roughness: 0.0,
    });
    let concrete = scene.push_material(concrete);

    // Ground
    let tri = Triangle3D::new(
        Point3D::new(-GROUND_SIZE, -GROUND_SIZE, 0.0),
        Point3D::new(GROUND_SIZE, -GROUND_SIZE, 0.0),
        Point3D::new(GROUND_SIZE, GROUND_SIZE, 0.0),
    )
    .unwrap();
    scene.push_object(concrete, concrete, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(-GROUND_SIZE, -GROUND_SIZE, 0.0),
        Point3D::new(GROUND_SIZE, GROUND_SIZE, 0.0),
        Point3D::new(-GROUND_SIZE, GROUND_SIZE, 0.0),
    )
    .unwrap();
    scene.push_object(concrete, concrete, Primitive::Triangle(tri));

    /* BASE */
    // Top of base
    let z = OFFSET + BASE_THICKNESS;
    let tri = Triangle3D::new(
        Point3D::new(0., 0., z),
        Point3D::new(WIDTH, 0.0, z),
        Point3D::new(0.0, DEPTH, z),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(WIDTH, 0.0, z),
        Point3D::new(WIDTH, DEPTH, z),
        Point3D::new(0.0, DEPTH, z),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    // Front
    let tri = Triangle3D::new(
        Point3D::new(WIDTH, DEPTH, OFFSET + BASE_THICKNESS),
        Point3D::new(0.0, DEPTH, OFFSET),
        Point3D::new(0.0, DEPTH, OFFSET + BASE_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(WIDTH, DEPTH, OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, DEPTH, OFFSET),
        Point3D::new(0.0, DEPTH, OFFSET),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    // BACK
    let tri = Triangle3D::new(
        Point3D::new(WIDTH, 0.0, OFFSET + BASE_THICKNESS),
        Point3D::new(0.0, 0.0, OFFSET),
        Point3D::new(0.0, 0.0, OFFSET + BASE_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(WIDTH, 0.0, OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, 0.0, OFFSET),
        Point3D::new(0.0, 0.0, OFFSET),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    // LEFT
    let tri = Triangle3D::new(
        Point3D::new(WIDTH, 0.0, OFFSET),
        Point3D::new(WIDTH, 0.0, OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, DEPTH, OFFSET + BASE_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(WIDTH, 0.0, OFFSET),
        Point3D::new(WIDTH, DEPTH, OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, DEPTH, OFFSET),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    // RIGHT
    let tri = Triangle3D::new(
        Point3D::new(0.0, 0.0, OFFSET),
        Point3D::new(0.0, 0.0, OFFSET + BASE_THICKNESS),
        Point3D::new(0.0, DEPTH, OFFSET + BASE_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(0.0, 0.0, OFFSET),
        Point3D::new(0.0, 0.0, OFFSET + BASE_THICKNESS),
        Point3D::new(0.0, DEPTH, OFFSET),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    /* SCREEN */

    let y: Float = DEPTH * ANGLE.cos();
    let z: Float = DEPTH * ANGLE.sin();
    // Top of screen
    let tri = Triangle3D::new(
        Point3D::new(0., 0., BASE_THICKNESS + SCREEN_THICKNESS + OFFSET),
        Point3D::new(WIDTH, 0.0, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
        Point3D::new(WIDTH, y, z + OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(0., 0., OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
        Point3D::new(WIDTH, y, z + OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
        Point3D::new(0.0, y, z + OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    // Left of screen
    let tri = Triangle3D::new(
        Point3D::new(WIDTH, 0., OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, y, z + OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
        Point3D::new(WIDTH, 0.0, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(WIDTH, 0., OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, y, z + OFFSET + SCREEN_THICKNESS),
        Point3D::new(WIDTH, y, z + OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    // right of screen
    let tri = Triangle3D::new(
        Point3D::new(0.0, 0., OFFSET + BASE_THICKNESS),
        Point3D::new(0.0, y, z + OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
        Point3D::new(0.0, 0.0, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(0.0, 0., OFFSET + BASE_THICKNESS),
        Point3D::new(0.0, y, z + OFFSET),
        Point3D::new(0.0, y, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    // back of screen
    let tri = Triangle3D::new(
        Point3D::new(0.0, 0.0, BASE_THICKNESS + OFFSET),
        Point3D::new(WIDTH, 0.0, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
        Point3D::new(0.0, 0.0, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(0.0, 0.0, BASE_THICKNESS + OFFSET),
        Point3D::new(WIDTH, 0.0, OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, 0.0, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    // Front of screen
    let tri = Triangle3D::new(
        Point3D::new(0.0, y, BASE_THICKNESS + OFFSET + z),
        Point3D::new(WIDTH, y, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS + z),
        Point3D::new(0.0, y, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS + z),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(0.0, y, 0.0 + BASE_THICKNESS + OFFSET + z),
        Point3D::new(WIDTH, y, OFFSET + BASE_THICKNESS + z),
        Point3D::new(WIDTH, y, OFFSET + BASE_THICKNESS + SCREEN_THICKNESS + z),
    )
    .unwrap();
    scene.push_object(plastic, plastic, Primitive::Triangle(tri));

    // Bottom of screen (a.k.a. Screen)
    let tri = Triangle3D::new(
        Point3D::new(0., 0., OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, y, z + OFFSET + BASE_THICKNESS),
        Point3D::new(0.0, y, z + OFFSET + BASE_THICKNESS),
    )
    .unwrap();
    scene.push_object(screen, screen, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(0., 0., OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, 0.0, OFFSET + BASE_THICKNESS),
        Point3D::new(WIDTH, y, z + OFFSET + BASE_THICKNESS),
    )
    .unwrap();
    scene.push_object(screen, screen, Primitive::Triangle(tri));

    scene.build_accelerator();

    // Create film
    let film = Film {
        resolution: (512, 512),
    };

    // Create view
    let view_point = Point3D::new(0.9, -0.4, 0.3);
    let view_direction = (Point3D::new(0., WIDTH / 2., DEPTH) - view_point).get_normalized();
    let view = View {
        view_direction,
        view_point,
        ..View::default()
    };

    // Create camera
    let camera = Pinhole::new(view, film);

    let integrator = RayTracer {
        n_ambient_samples: 180,
        n_shadow_samples: 38,
        max_depth: 3,
        ..RayTracer::default()
    };

    let buffer = integrator.render(&scene, &camera);
    buffer.save_hdre(std::path::Path::new("./test_data/images/laptop.hdr"));
}

use geometry3d::Point3D;
use rendering::camera::{Film, Pinhole, View};
use rendering::ray_tracer::RayTracer;

use geometry3d::{Sphere3D, Triangle3D, Vector3D};
use rendering::colour::Spectrum;
use rendering::material::*;
use rendering::primitive::Primitive;
use rendering::scene::Scene;
use rendering::{Float, PI};

#[test]
// #[ignore]
fn laptop() {
    // cargo test --features parallel --release --package rendering --test test_scenes -- laptop --exact --nocapture
    let mut scene = Scene::new();

    const BASE_THICKNESS: Float = 0.01;
    const SCREEN_THICKNESS: Float = 0.006;
    const WIDTH: Float = 0.38;
    const DEPTH: Float = 0.3;
    const ANGLE: Float = 30. * PI / 180.0;
    const GROUND_SIZE: Float = 10.;
    const OFFSET: Float = 0.002;

    // Add light
    let glow = scene.push_material(Material::Light(Light(
        Spectrum {
            red: 1.,
            green: 1.,
            blue: 1.,
        } * 500.,
    )));

    let s = Sphere3D::new(0.1, Point3D::new(0., 0., 5.));
    scene.push_object(glow, glow, Primitive::Sphere(s));

    // Materials
    let plastic = Material::Plastic(Plastic {
        colour: Spectrum {
            red: 0.5,
            green: 0.8,
            blue: 0.5,
        },
        specularity: 0.0,
        roughness: 0.0,
    });
    let plastic = scene.push_material(plastic);

    let screen = Material::Light(Light(
        Spectrum {
            red: 145.,
            green: 7.,
            blue: 205.,
        } * 0.3,
    ));
    let screen = scene.push_material(screen);

    let ground = Material::Plastic(Plastic {
        colour: Spectrum {
            red: 0.2,
            green: 0.2,
            blue: 0.2,
        },
        specularity: 0.0,
        roughness: 0.01,
    });
    let ground = scene.push_material(ground);

    // Ground
    let tri = Triangle3D::new(
        Point3D::new(-GROUND_SIZE, -GROUND_SIZE, 0.0),
        Point3D::new(GROUND_SIZE, -GROUND_SIZE, 0.0),
        Point3D::new(GROUND_SIZE, GROUND_SIZE, 0.0),
    )
    .unwrap();
    scene.push_object(ground, ground, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(-GROUND_SIZE, -GROUND_SIZE, 0.0),
        Point3D::new(GROUND_SIZE, GROUND_SIZE, 0.0),
        Point3D::new(-GROUND_SIZE, GROUND_SIZE, 0.0),
    )
    .unwrap();
    scene.push_object(ground, ground, Primitive::Triangle(tri));

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


    // Tests
    // let exp_materials = vec![
    //     ground, ground, // ground
    //     plastic, plastic, // top of base
    //     plastic, plastic,  // Front
    //     plastic, plastic,  // back
    //     plastic, plastic,  // left
    //     plastic, plastic,  // right
    //     plastic, plastic,  // Top of screen
    //     plastic, plastic,  // Left of screen
    //     plastic, plastic,  // right of screen
    //     plastic, plastic,  // back of screen
    //     plastic, plastic,  // Front of screen
    //     screen, screen,  // Screen
    // ];
    // assert_eq!(exp_materials, scene.front_material_indexes);
    // assert_eq!(exp_materials, scene.back_material_indexes);

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
        n_ambient_samples: 220,
        n_shadow_samples: 10,
        max_depth: 3,
        ..RayTracer::default()
    };

    let buffer = integrator.render(&scene, &camera);
    buffer.save_hdre(std::path::Path::new("./test_data/images/laptop.hdr"));
}


#[test]
#[ignore]
fn sponza(){
    // cargo test --features parallel --release --package rendering --test test_scenes -- sponza --exact --nocapture

    let mut scene = Scene::default();
        let gray = scene.push_material(Material::Plastic(Plastic{
            colour: Spectrum::gray(0.3),
            specularity: 0., 
            roughness: 0.,
        }));

    scene.add_from_obj("./test_data/sponza.obj".to_string(), gray, gray);

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
    let view_point = Point3D::new(0.0, 5., 0.0);
    let view_direction = Vector3D::new(1., 0., 0.).get_normalized();
    let view = View {
        view_direction,
        view_point,
        view_up : Vector3D::new(0., 1., 0.),
        ..View::default()
    };

    // Create camera
    let camera = Pinhole::new(view, film);

    let integrator = RayTracer {
        n_ambient_samples: 80,
        n_shadow_samples: 1,
        max_depth: 2,
        ..RayTracer::default()
    };

    let buffer = integrator.render(&scene, &camera);
    buffer.save_hdre(std::path::Path::new("./test_data/images/sponza.hdr"));

}




#[test]
#[ignore]
fn cornell() {
    // 60 seconds
    // cargo test --features parallel --release --package rendering --test test_scenes -- cornell --exact --nocapture
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
        n_ambient_samples: 80,
        n_shadow_samples: 5,
        max_depth: 1,
        // count_specular_bounce: 0.1,
        ..RayTracer::default()
    };

    let buffer = integrator.render(&scene, &camera);
    buffer.save_hdre(std::path::Path::new("./test_data/images/cornell.hdr"));
}



#[test]
#[ignore]
fn room() {
    // 60 seconds
    // cargo test --features parallel --release --package rendering --test test_scenes -- room --exact --nocapture
    // oconv ../room.rad ../white_sky.rad > room.oct ;time rpict -x 512 -y 512 -vv 60 -vh 60 -ab 3 -ad 220 -aa 0 -vp 2 1 1 -vd 0 1 0 ./room.oct > rad_room.hdr

    let mut scene = Scene::from_radiance("./test_data/room.rad".to_string());
    // scene.add_perez_sky(
    //     calendar::Date {
    //         month: 6,
    //         day: 1,
    //         hour: 12.,
    //     },
    //     -33.,
    //     70.,
    //     65.,
    //     200.,
    //     500.,
    // );

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


#[test]
#[ignore]
fn dining(){
    // cargo test --features parallel --release --package rendering --test test_scenes -- dining --exact --nocapture

    let mut scene = Scene::default();
        let gray = scene.push_material(Material::Plastic(Plastic{
            colour: Spectrum::gray(0.3),
            specularity: 0., 
            roughness: 0.,
        }));

    scene.add_from_obj("./test_data/casa2.obj".to_string(), gray, gray);
    
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
        resolution: (830, 550),
    };

    // Create view
    let view_point = Point3D::new(-4.0, 1.3, 0.);
    let view_direction = Vector3D::new(1., -0.12, 0.).get_normalized();
    let view = View {
        view_direction,
        view_point,
        field_of_view: 48.,
        
        view_up : Vector3D::new(0., 1., 0.),
        ..View::default()
    };

    // Create camera
    let camera = Pinhole::new(view, film);

    let integrator = RayTracer {
        n_ambient_samples: 60,
        n_shadow_samples: 1,
        max_depth: 1,
        ..RayTracer::default()
    };

    let buffer = integrator.render(&scene, &camera);
    buffer.save_hdre(std::path::Path::new("./test_data/images/dining.hdr"));

}
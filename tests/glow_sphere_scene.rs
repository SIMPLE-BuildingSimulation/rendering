use geometry3d::{Point3D, Sphere3D, Triangle3D};
use rendering::colour::Spectrum;
use rendering::material::*;
use rendering::primitive::Primitive;
use rendering::scene::Scene;
use rendering::Float;

pub fn get_scene(scene_height: Float, ground_size: Float) -> Scene {
    let mut scene = Scene::new();

    // let mirror = scene.push_material(Box::new(Mirror(Spectrum {
    //     red: 0.99,
    //     green: 0.99,
    //     blue: 0.99,
    // })));

    let glow = scene.push_material(Box::new(Light(
        //145, 7, 205
        Spectrum {
            red: 145.,
            green: 7.,
            blue: 205.,
        } * 1.,
    )));

    let gray = scene.push_material(Box::new(Plastic {
        colour: Spectrum {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
        },
        specularity: 0.,
        roughness: 0.,
    }));

    // Ground

    let tri = Triangle3D::new(
        Point3D::new(-ground_size, -ground_size, scene_height),
        Point3D::new(ground_size, -ground_size, scene_height),
        Point3D::new(ground_size, ground_size, scene_height),
    )
    .unwrap();
    scene.push_object(gray, gray, Primitive::Triangle(tri));
    let tri = Triangle3D::new(
        Point3D::new(-ground_size, -ground_size, scene_height),
        Point3D::new(ground_size, ground_size, scene_height),
        Point3D::new(-ground_size, ground_size, scene_height),
    )
    .unwrap();
    scene.push_object(gray, gray, Primitive::Triangle(tri));

    // Objects
    let s = Sphere3D::new_partial(
        1.5,
        Point3D::new(0., 0., 1.5 + scene_height),
        -1.5,
        -0.05,
        360.,
    );
    scene.push_object(gray, glow, Primitive::Sphere(s));

    let s = Sphere3D::new_partial(
        1.5,
        Point3D::new(0., 0., 1.5 + scene_height),
        0.05,
        1.5,
        360.,
    );
    scene.push_object(gray, glow, Primitive::Sphere(s));

    // scene.add_perez_sky(
    //     calendar::Date{month: 1, day: 1, hour: 13.,},
    //     -33.,
    //     70.,
    //     65.,
    //     200., 500.
    // );

    // assert_eq!(scene.count_all_lights(), 2);

    // return
    scene
}

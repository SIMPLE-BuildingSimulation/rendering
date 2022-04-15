use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rendering::rand::Rng;

// Reference targets: https://github.com/svenstaro/bvh
pub fn criterion_benchmark(c: &mut Criterion) {
    
    
    // Setup
    let mut aux : Vec<usize> = vec![0; 10];
    let mut ray = black_box(rendering::ray::Ray{
        geometry: geometry3d::Ray3D {
            direction: geometry3d::Vector3D::new(0., 1., 2.).get_normalized(),
            origin: geometry3d::Point3D::new(1., 2., 1.),
        },
        .. rendering::ray::Ray::default()
    });
    

    // ROOM
    
    let mut room = black_box(rendering::scene::Scene::from_radiance("./test_data/room.rad".to_string()));
    room.build_accelerator();

    c.bench_function("intersect_room", |b| {
        b.iter(|| {
            room.cast_ray(&mut ray, &mut aux)
        })
    });

    // c.bench_function("unobstructed_room", |b| {
    //     b.iter(|| {
    //         room.unobstructed_distance(&ray.geometry, rendering::Float::MAX, &mut aux)
    //     })
    // });


    // CORNELL

    // let mut cornell = black_box(rendering::scene::Scene::from_radiance("./test_data/cornell.rad".to_string()));
    // cornell.build_accelerator();    
    
    
    // c.bench_function("intersect_cornell", |b| {
    //     b.iter(|| {
    //         cornell.cast_ray(&mut ray, &mut aux)
    //     })
    // });

    
    // c.bench_function("unobstructed_cornell", |b| {
    //     b.iter(|| {
    //         cornell.unobstructed_distance(&ray.geometry, rendering::Float::MAX, &mut aux)
    //     })
    // });


    // TRIANGLES
    let mut triangles = black_box(rendering::scene::Scene::new());
    let plastic = rendering::material::Material::Plastic(rendering::material::Plastic {
        colour: rendering::colour::Spectrum {
            red: 0.5,
            green: 0.5,
            blue: 0.5,
        },
        specularity: 0.05,
        roughness: 0.1,
    });
    let mut rng = rendering::rand::get_rng();
    let plastic = triangles.push_material(plastic);
    let mut i = 0;
    while i < 120_000 {
        let (x1, y1, z1, x2, y2, z2, x3, y3, z3) : (rendering::Float, rendering::Float, rendering::Float, rendering::Float, rendering::Float, rendering::Float, rendering::Float, rendering::Float, rendering::Float) = rng.gen();

        const SCALE : rendering::Float = 30.;
        if let Ok(tri) = geometry3d::Triangle3D::new(
            geometry3d::Point3D::new((x1-0.5)*SCALE, (y1-0.5)*SCALE, (z1-0.5)*SCALE),
            geometry3d::Point3D::new((x2-0.5)*SCALE, (y2-0.5)*SCALE, (z2-0.5)*SCALE),
            geometry3d::Point3D::new((x3-0.5)*SCALE, (y3-0.5)*SCALE, (z3-0.5)*SCALE),
        ){
            i +=1;
            triangles.push_object(plastic, plastic, rendering::primitive::Primitive::Triangle(tri));
        };

    }
    triangles.build_accelerator();

    c.bench_function("intersect_triangles", |b| {
        b.iter(|| {
            triangles.cast_ray(&mut ray, &mut aux)
        })
    });

    
    // c.bench_function("unobstructed_triangles", |b| {
    //     b.iter(|| {
    //         triangles.unobstructed_distance(&ray.geometry, rendering::Float::MAX, &mut aux)
    //     })
    // }

}



criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Reference targets: https://github.com/svenstaro/bvh
pub fn criterion_benchmark(c: &mut Criterion) {
    let mut room = black_box(rendering::scene::Scene::from_radiance("./test_data/room.rad".to_string()));
    room.build_accelerator();
    let mut cornell = black_box(rendering::scene::Scene::from_radiance("./test_data/cornell.rad".to_string()));
    cornell.build_accelerator();
    let mut aux : Vec<usize> = vec![0; 10];
    let mut ray = black_box(rendering::ray::Ray{
        geometry: geometry3d::Ray3D {
            direction: geometry3d::Vector3D::new(0., 1., 2.).get_normalized(),
            origin: geometry3d::Point3D::new(1., 2., 1.),
        },
        .. rendering::ray::Ray::default()
    });
    
    c.bench_function("intersect_room", |b| {
        b.iter(|| {
            room.cast_ray(&mut ray, &mut aux)
        })
    });

    c.bench_function("intersect_cornell", |b| {
        b.iter(|| {
            cornell.cast_ray(&mut ray, &mut aux)
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);


use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rendering::samplers::local_to_world;
use geometry3d::{Point3D,Vector3D};


pub fn criterion_benchmark(c: &mut Criterion) {
    let normal = black_box(Vector3D::new(0., 1., 0.));
    let pt = black_box(Point3D::new(0., 0., 0.));
    let e2 = normal.get_perpendicular().unwrap();
    let e1 = e2.cross(normal);
    c.bench_function("local_to_world", |b| b.iter(|| local_to_world(
        normal, e1, e2, pt,
        black_box(9.), black_box(19.), black_box(11.), 
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
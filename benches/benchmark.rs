
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rendering::samplers::local_to_world;
use geometry3d::{Point3D,Vector3D};


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("local_to_world", |b| b.iter(|| local_to_world(
        black_box(Vector3D::new(0., 1., 0.)),
        black_box(Point3D::new(0., 0., 0.)),
        black_box(9.), black_box(19.), black_box(11.), 
    )));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
use criterion::{/*black_box,*/ criterion_group, criterion_main, Criterion};
use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;
use rendering::camera::{Camera, View};
use rendering::ray_caster::RayCaster;
use rendering::scene::Scene;
use std::time::Duration;

//use rendering::from_radiance::

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("render_scenes");
    // compare_with_radiance("exterior_0_diffuse_plastic.rad".to_string());
    // compare_with_radiance("exterior_0_specularity.rad".to_string());
    // compare_with_radiance("exterior_0_mirror.rad".to_string());
    // compare_with_radiance("exterior_0_dielectric.rad".to_string());
    group
        .significance_level(0.99)
        .sample_size(10)
        .measurement_time(Duration::from_secs(9000));
    group.bench_function("exterior_0_diffuse_plastic", |b| {
        b.iter(|| compare_with_radiance("exterior_0_diffuse_plastic.rad".to_string()))
    });
    group.bench_function("exterior_0_specularity", |b| {
        b.iter(|| compare_with_radiance("exterior_0_specularity.rad".to_string()))
    });
    group.bench_function("exterior_0_mirror", |b| {
        b.iter(|| compare_with_radiance("exterior_0_mirror.rad".to_string()))
    });
    // c.bench_function("exterior_0_dielectric", |b| b.iter(|| compare_with_radiance("exterior_0_dielectric.rad".to_string()) ));
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

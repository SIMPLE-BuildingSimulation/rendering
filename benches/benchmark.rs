
use criterion::{black_box, criterion_group, criterion_main, Criterion};
type Float = f64;
use rendering::rand::*;
const PI : Float = std::f64::consts::PI;








// use rand::distributions::{Distribution, Uniform};
pub fn criterion_benchmark(c: &mut Criterion) {
    
    
    // let mut rng = black_box(rand::thread_rng());
    let rng_src = black_box(get_rng());
    let mut rng = clone_rng(&rng_src);

    
    c.bench_function("uniform_sample_horizontal_disc", |b| b.iter(|| 
        rendering::samplers::uniform_sample_horizontal_disc(&mut rng, black_box(1.))        
    ));
    c.bench_function("sample_cosine_weighted_horizontal_hemisphere", |b| b.iter(|| 
        rendering::samplers::sample_cosine_weighted_horizontal_hemisphere(&mut rng)        
    ));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
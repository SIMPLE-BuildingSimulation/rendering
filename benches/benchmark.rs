
use criterion::{black_box, criterion_group, criterion_main, Criterion};



use rendering::samplers::{HorizontalDiskUniformSampler, HorizontalCosineWeightedHemisphereSampler};




// use rand::distributions::{Distribution, Uniform};
pub fn criterion_benchmark(c: &mut Criterion) {
    
    


    
    let mut sampler = black_box(HorizontalDiskUniformSampler::new(1., std::usize::MAX));
    
    c.bench_function("uniform_sample_horizontal_disc", |b| b.iter(|| {
        // let mut rng = black_box(get_rng());//clone_rng(&rng_src);
        // Choose a direction.            
        // let new_ray_dir = rendering::samplers::sample_cosine_weighted_horizontal_hemisphere(&mut rng);        
        // rendering::samplers::uniform_sample_horizontal_disc(&mut rng, black_box(1.))        
        sampler.next()
    }));

    let mut sampler = black_box(HorizontalCosineWeightedHemisphereSampler::new(std::usize::MAX));
    c.bench_function("sample_cosine_weighted_horizontal_hemisphere", |b| b.iter(|| {

        // let mut rng = black_box(get_rng());
        // rendering::samplers::sample_cosine_weighted_horizontal_hemisphere(&mut rng)        
        sampler.next()
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
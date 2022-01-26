
use criterion::{black_box, criterion_group, criterion_main, Criterion};


use rendering::ray::Ray;
use rendering::material::*;
use rendering::colour::Spectrum;
use geometry3d::{Vector3D, Point3D, Ray3D};

use rand::prelude::*;


// use rand::distributions::{Distribution, Uniform};
pub fn criterion_benchmark(c: &mut Criterion) {
    




    /* MATERIALS */
    let mut rng = black_box(rendering::rand::get_rng());
    let e1 = black_box(Vector3D::new(1., 0., 0.));
    let e2 = black_box(Vector3D::new(0., 1., 0.));
    let normal = black_box(Vector3D::new(0., 0., 1.));
    let ray = black_box(Ray{
        geometry: Ray3D{
            direction: Vector3D::new(1., 2., 3.).get_normalized(),
            origin: Point3D::new(1., 2., 3.),
        },
        refraction_index: 1.
    });
    let vout = black_box(Vector3D::new(1., 4., 12.).get_normalized());




    
    let plastic = black_box(Material::Plastic(PlasticMetal{
        color: Spectrum{red: 0.5, green: 0.2, blue: 0.9},
        specularity: 0.0, 
        roughness: 0.0
    }));
    c.bench_function("sample_plastic", |b| b.iter(|| {
        let (_new_ray, _pdf, _is_specular ) =plastic.sample_bsdf(normal, e1, e2, Point3D::new(0., 0., 0.), ray, &mut rng);
    }));
    c.bench_function("eval_plastic", |b| b.iter(|| {
        let _val = plastic.eval_bsdf(normal, e1, e2, ray, vout);
    }));



    let metal = black_box(Material::Metal(PlasticMetal{
        color: Spectrum{red: 0.5, green: 0.2, blue: 0.9},
        specularity: 0.0, 
        roughness: 0.0
    }));
    c.bench_function("sample_metal", |b| b.iter(|| {
        let (_new_ray, _pdf, _is_specular ) = metal.sample_bsdf(normal, e1, e2, Point3D::new(0., 0., 0.), ray, &mut rng);
    }));

    c.bench_function("eval_metal", |b| b.iter(|| {
        let _val = metal.eval_bsdf(normal, e1, e2, ray, vout);
    }));




    let mirror = black_box(Material::Mirror(Spectrum{red: 0.5, green: 0.2, blue: 0.9}));
    c.bench_function("sample_mirror", |b| b.iter(|| {
        let (_new_ray, _pdf, _is_specular ) = mirror.sample_bsdf(normal, e1, e2, Point3D::new(0., 0., 0.), ray, &mut rng);
    }));
    c.bench_function("eval_mirror", |b| b.iter(|| {
        let _val = mirror.eval_bsdf(normal, e1, e2, ray, vout);
    }));


    

    let dielectric = black_box(Material::Dielectric(Dielectric{
        color: Spectrum{red: 0.5, green: 0.2, blue: 0.9},
        refraction_index: 1.5, 
        
    }));
    c.bench_function("sample_dielectric", |b| b.iter(|| {
        let (_new_ray, _pdf, _is_specular ) = dielectric.sample_bsdf(normal, e1, e2, Point3D::new(0., 0., 0.), ray, &mut rng);
    }));
    c.bench_function("eval_dielectric", |b| b.iter(|| {
        let _val = dielectric.eval_bsdf(normal, e1, e2, ray, vout);
    }));

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
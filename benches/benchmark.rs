use criterion::{black_box, criterion_group, criterion_main, Criterion};

use geometry3d::{Point3D, Ray3D, Vector3D};
use rendering::colour::Spectrum;
use rendering::material::*;
use rendering::ray::Ray;

use rendering::rand::*;
use rendering::samplers::*;

type Float = f64;

pub fn this_local_to_world(
    local_e1: Vector3D,
    local_e2: Vector3D,
    normal: Vector3D,
    centre: Point3D,
    x_local: Float,
    y_local: Float,
    z_local: Float,
) -> (Float, Float, Float) {
    // Check that they are normalized
    debug_assert!((1. - local_e1.length_squared()).abs() < 1e-4);
    debug_assert!((1. - local_e2.length_squared()).abs() < 1e-4);
    debug_assert!((1. - normal.length_squared()).abs() < 1e-4);

    let x = centre.x + x_local * local_e1.x + y_local * local_e2.x + z_local * normal.x;
    let y = centre.y + x_local * local_e1.y + y_local * local_e2.y + z_local * normal.y;
    let z = centre.z + x_local * local_e1.z + y_local * local_e2.z + z_local * normal.z;

    (x, y, z)
}

// use rand::distributions::{Distribution, Uniform};
pub fn criterion_benchmark(c: &mut Criterion) {
    /* ********* */
    /* SAMPLERS */
    /* ********* */
    // let mut rng = black_box(get_rng());
    // c.bench_function("SmallRng", move |b| {
    //     b.iter(|| {
    //         let _a : f64 = rng.gen();
    //     })
    // });

    // let mut rng = black_box(get_rng());
    // c.bench_function("uniform_sample_horizontal_disc", |b| {
    //     b.iter(|| {
    //         let (_a,_b) = uniform_sample_horizontal_disc(black_box(&mut rng), 1.);
    //     })
    // });

    // let mut rng = black_box(get_rng());
    // c.bench_function("sample_cosine_weighted_horizontal_hemisphere", |b| {
    //     b.iter(|| {
    //         let _a = sample_cosine_weighted_horizontal_hemisphere(black_box(&mut rng));
    //     })
    // });

    // let e1 = black_box(Vector3D::new(1., 0., 0.).get_normalized());
    // let e2 = black_box(Vector3D::new(0., 1., 0.).get_normalized());
    // let normal = black_box(Vector3D::new(0., 0., 1.));
    // let centre = black_box(Point3D::new(0., 1., 2.));
    // let (x_local, y_local, z_local) = black_box((241., 12., 64.));

    // c.bench_function("local_to_world", |b| {
    //     b.iter(|| {
    //         let (x, y, z) = local_to_world(black_box(e1), black_box(e2), black_box(normal), black_box(centre), black_box(x_local), black_box(y_local), black_box(z_local));
    //         let _dir = Vector3D::new(x, y, z);
    //     })
    // });

    /* ********* */
    /* MATERIALS */
    /* ********* */
    let mut rng = black_box(rendering::rand::get_rng());
    let e1 = black_box(Vector3D::new(1., 0., 0.));
    let e2 = black_box(Vector3D::new(0., 1., 0.));
    let normal = black_box(Vector3D::new(0., 0., 1.));
    let ray = black_box(Ray {
        geometry: Ray3D {
            direction: Vector3D::new(1., 2., 3.).get_normalized(),
            origin: Point3D::new(1., 2., 3.),
        },
        refraction_index: 1.,
    });
    let vout = black_box(Vector3D::new(1., 4., 12.).get_normalized());

    let p = PlasticMetal {
        color: Spectrum {
            red: 0.5,
            green: 0.2,
            blue: 0.9,
        },
        specularity: 0.0,
        roughness: 0.0,
    };
    c.bench_function("direct_sample_plastic", |b| {
        b.iter(|| {
            let (_new_ray, _pdf, _is_specular) = p.bsdf(
                black_box(normal),
                black_box(e1),
                black_box(e2),
                black_box(Point3D::new(0., 0., 0.)),
                black_box(ray),
                black_box(&mut rng),
            );
        })
    });

    let p = PlasticMetal {
        color: Spectrum {
            red: 0.5,
            green: 0.2,
            blue: 0.9,
        },
        specularity: 0.0,
        roughness: 0.0,
    };

    let plastic = black_box(Material::Plastic(p));
    c.bench_function("sample_plastic", |b| {
        b.iter(|| {
            let (_new_ray, _pdf, _is_specular) = plastic.sample_bsdf(
                black_box(normal),
                black_box(e1),
                black_box(e2),
                black_box(Point3D::new(0., 0., 0.)),
                black_box(ray),
                black_box(&mut rng),
            );
        })
    });

    c.bench_function("eval_plastic", |b| {
        b.iter(|| {
            let _val = plastic.eval_bsdf(normal, e1, e2, ray, vout);
        })
    });

    let metal = black_box(Material::Metal(PlasticMetal {
        color: Spectrum {
            red: 0.5,
            green: 0.2,
            blue: 0.9,
        },
        specularity: 0.0,
        roughness: 0.0,
    }));
    c.bench_function("sample_metal", |b| {
        b.iter(|| {
            let (_new_ray, _pdf, _is_specular) = metal.sample_bsdf(
                black_box(normal),
                black_box(e1),
                black_box(e2),
                black_box(Point3D::new(0., 0., 0.)),
                black_box(ray),
                black_box(&mut rng),
            );
        })
    });

    c.bench_function("eval_metal", |b| {
        b.iter(|| {
            let _val = metal.eval_bsdf(normal, e1, e2, ray, vout);
        })
    });

    let mirror = black_box(Material::Mirror(Spectrum {
        red: 0.5,
        green: 0.2,
        blue: 0.9,
    }));
    c.bench_function("sample_mirror", |b| {
        b.iter(|| {
            let (_new_ray, _pdf, _is_specular) =
                mirror.sample_bsdf(black_box(normal), black_box(e1), black_box(e2), black_box(Point3D::new(0., 0., 0.)), black_box(ray), black_box(&mut rng));
        })
    });
    c.bench_function("get_possible_paths_mirror", |b| {
        b.iter(|| {
            let _a = mirror.get_possible_paths(
                black_box(&normal),
                black_box(&Point3D::new(0., 0., 0.)),
                black_box(&ray),
            );
        })
    });
    c.bench_function("eval_mirror", |b| {
        b.iter(|| {
            let _val = mirror.eval_bsdf(normal, e1, e2, ray, vout);
        })
    });

    let dielectric = black_box(Material::Dielectric(Dielectric {
        color: Spectrum {
            red: 0.5,
            green: 0.2,
            blue: 0.9,
        },
        refraction_index: 1.5,
    }));
    c.bench_function("get_possible_paths_dielectric", |b| {
        b.iter(|| {
            let _a = dielectric.get_possible_paths(
                black_box(&normal),
                black_box(&Point3D::new(0., 0., 0.)),
                black_box(&ray),
            );
        })
    });
    // c.bench_function("sample_dielectric", |b| {
    //     b.iter(|| {
    //         let (_new_ray, _pdf, _is_specular) =
    //             dielectric.sample_bsdf(black_box(normal), black_box(e1), black_box(e2), black_box(Point3D::new(0., 0., 0.)), black_box(ray), black_box(&mut rng));
    //     })
    // });
    c.bench_function("eval_dielectric", |b| {
        b.iter(|| {
            let _val = dielectric.eval_bsdf(normal, e1, e2, ray, vout);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

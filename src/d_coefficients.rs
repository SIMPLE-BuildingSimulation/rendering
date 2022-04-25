/*
MIT License
Copyright (c) 2021 Germán Molina
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use crate::ray_tracer::RayTracerHelper;
use crate::colour_matrix::ColourMatrix;
use crate::Float;
use geometry3d::intersection::SurfaceSide;
use geometry3d::Vector3D;
use geometry3d::{Point3D, Ray3D};
use crate::colour::Spectrum;
use crate::rand::*;
use crate::ray::Ray;
use crate::samplers::sample_cosine_weighted_horizontal_hemisphere;
use crate::scene::Scene;
use solar::ReinhartSky;

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// A structure meant to calculate DC matrices
/// for Climate Daylight Simulations.
pub struct DCFactory {
    pub reinhart: ReinhartSky,
    pub max_depth: usize,
    pub n_ambient_samples: usize,
    pub limit_weight: Float,
    pub count_specular_bounce: Float,
    // pub limit_reflections: usize,
}

impl Default for DCFactory {
    fn default() -> Self {
        Self {
            reinhart: ReinhartSky::new(1),
            max_depth: 0,
            n_ambient_samples: 10,
            count_specular_bounce: 0.5,

            limit_weight: 1e-4,
            // limit_reflections: 0,
        }
    }
}

impl DCFactory {
    pub fn calc_dc(&self, rays: &[Ray3D], scene: &Scene) -> ColourMatrix {
        // Initialize matrix
        let n_bins = self.reinhart.n_bins;

        // Process... This can be in parallel, or not.
        #[cfg(not(feature = "parallel"))]
        let aux_iter = rays.iter();
        #[cfg(feature = "parallel")]
        let aux_iter = rays.par_iter();
        // Iterate the rays
        let dcs: Vec<ColourMatrix> = aux_iter
            .map(|primary_ray| -> ColourMatrix {
                let normal = primary_ray.direction;
                let origin = primary_ray.origin;
                let e2 = normal.get_perpendicular().unwrap();
                let e1 = e2.cross(normal);

                // Run each spawned ray in parallel or series, depending on
                // the compilation options
                let mut rng = get_rng();
                let aux_iter : Vec<Vector3D> = (0..self.n_ambient_samples).into_iter().map(|_|{
                    sample_cosine_weighted_horizontal_hemisphere(&mut rng)
                }).collect();
                
                #[cfg(not(feature = "parallel"))]
                let aux_iter = aux_iter.into_iter();

                #[cfg(feature = "parallel")]
                let aux_iter = aux_iter.into_par_iter();

                // Iterate primary rays
                let ray_contributions: Vec<ColourMatrix> = aux_iter
                    .map(|local_ray_dir: Vector3D| -> ColourMatrix {
                        let (x, y, z) = crate::samplers::local_to_world(
                            e1,
                            e2,
                            normal,
                            Point3D::new(0., 0., 0.),
                            local_ray_dir.x,
                            local_ray_dir.y,
                            local_ray_dir.z,
                        );
                        let new_ray_dir = Vector3D::new(x, y, z);

                        let mut this_ret = ColourMatrix::new(Spectrum::black(), 1, n_bins);

                        debug_assert!(
                            (1. - new_ray_dir.length()).abs() < 0.0000001,
                            "length is {}",
                            new_ray_dir.length()
                        );

                        let mut aux = RayTracerHelper::default();
                        let mut new_ray = Ray {
                            // time: 0.,
                            geometry: Ray3D {
                                direction: new_ray_dir,
                                origin,
                            },
                            colour: Spectrum::gray(1.),
                            .. Ray::default()
                        };

                        let mut rng = get_rng();
                        // let current_weight = cos_theta;
                        self.trace_ray(
                            scene,
                            &mut new_ray,                            
                            self.n_ambient_samples,
                            &mut this_ret,
                            &mut rng,
                            &mut aux,
                        );

                        // let mut c = counter.lock().unwrap();
                        // *c += 1;
                        // let nrays = rays.len() * factory.n_ambient_samples;
                        // let perc = (100. *  *c as Float/ nrays  as Float).round() as usize;
                        // eprintln!("Ray {} of {} ({}%) done...", c, nrays, perc);

                        this_ret
                    })
                    .collect(); // End of iterating primary rays

                let mut ret = ColourMatrix::new(Spectrum::black(), 1, n_bins);
                ray_contributions.iter().for_each(|v| {
                    ret += v;
                });
                ret
                // ray_contributions.iter().sum();
            })
            .collect(); // End of iterating rays

        // Write down the results
        let mut ret = ColourMatrix::new(Spectrum::black(), rays.len(), n_bins);
        for (sensor_index, contribution) in dcs.iter().enumerate() {
            // add contribution
            for patch_index in 0..n_bins {
                let v = contribution.get(0, patch_index).unwrap();
                ret.set(sensor_index, patch_index, v).unwrap();
            }
        }

        ret
    }

    /// Recursively traces a ray until it excedes the `max_depth` of the
    /// `DCFactory` or the ray does not hit anything (i.e., it reaches either
    /// the sky or the ground)
    fn trace_ray(
        &self,
        scene: &Scene,
        ray: &mut Ray,
        accum_denom_samples: usize,
        contribution: &mut ColourMatrix,
        rng: &mut RandGen,
        aux: &mut RayTracerHelper,
    ) {

        // Limit bounces
        if ray.depth > self.max_depth {
            return;
        }

        // let one_over_samples = 1./ self.n_ambient_samples as Float;
        // If hits an object
        if let Some(triangle_index) = scene.cast_ray(ray, &mut aux.nodes) {

                // NEARLY copied... except from the return statement
                let material = match ray.interaction.geometry_shading.side {
                    SurfaceSide::Front => {
                        &scene.materials[scene.front_material_indexes[triangle_index]]
                    }
                    SurfaceSide::Back => {
                        &scene.materials[scene.back_material_indexes[triangle_index]]
                    },
                    SurfaceSide::NonApplicable => {
                        // Hit parallel to the surface...
                        return
                    }
                };
                

                // Limit bounces... also, emmiting materials don't reflect
                if ray.depth > self.max_depth || material.emits_direct_light(){
                    return;
                }

                let (intersection_pt, normal, ..) = ray.get_triad();
                ray.interaction.interpolate_normal(scene.normals[triangle_index]);
            

                // Handle specular materials... we have 1 or 2 rays... spawn those.
                if material.specular_only() {
                    
                    let paths = material.get_possible_paths(&normal, &intersection_pt, ray);
                    for (new_ray, _bsdf_value) in paths.iter().flatten() {
                        let mut new_ray = *new_ray;

                        // let new_ray_dir = new_ray.geometry.direction;
                        // let cos_theta = (normal * new_ray_dir).abs();
                        // new_ray.value *= bsdf_value * cos_theta * 1.5  ;

                        // avoid infinite interior bouncing
                        let q: Float = rng.gen();
                        if q < self.count_specular_bounce {
                            ray.depth += 1
                        } 
                        
                        // self.trace_ray(rng, scene, &mut new_ray, new_depth, new_value, aux);
                        self.trace_ray(scene, &mut new_ray, accum_denom_samples, contribution, rng, aux)
                        
                    }
                    return 
                }

                
                let n_ambient_samples = ray.get_n_ambient_samples(self.n_ambient_samples, self.max_depth, self.limit_weight);

                // Spawn more rays
                let depth = ray.depth;
                aux.rays[depth] = *ray;
                let (_pt, normal, e1, e2, ..) = ray.get_triad();
                (0..n_ambient_samples).for_each(|_| {
                                                
                    let (bsdf_value, pdf) = material.sample_bsdf(normal, e1, e2, intersection_pt, ray, rng);                    
                    let new_ray_dir = ray.geometry.direction;
                    debug_assert!((1. as Float-new_ray_dir.length()).abs() < 1e-5, "Length is {}", new_ray_dir.length());

                    // increase depth
                    let cos_theta = (normal * new_ray_dir).abs();
                    dbg!(pdf);
                    ray.colour *= bsdf_value * cos_theta * 1.5; // / pdf;
                    ray.depth += 1;

                                    
                    
                    self.trace_ray(scene, ray,  accum_denom_samples * n_ambient_samples, contribution, rng, aux);
                }); // End the foreach spawned ray
            
            
        } else {
            let bin_n = self.reinhart.dir_to_bin(ray.geometry.direction);

            let li = Spectrum::gray(3.);//*self.reinhart.bin_solid_angle(bin_n);
            let old_value = contribution.get(0, bin_n).unwrap();

            contribution
                .set(
                    0,
                    bin_n,
                    old_value + li * ray.colour / accum_denom_samples as Float,
                )
                .unwrap();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use geometry3d::{Point3D, Vector3D};
    #[test]
    #[ignore]
    fn test_calc_dc() {
        assert!(true);
        // return;
        // Setup sensors
        let up = Vector3D::new(0., 0., 1.);
        let rays = vec![
            Ray3D {
                origin: Point3D::new(2., 0.5, 0.8),
                direction: up,
            },
            Ray3D {
                origin: Point3D::new(2., 2.5, 0.8),
                direction: up,
            },
            Ray3D {
                origin: Point3D::new(2., 5.5, 0.8),
                direction: up,
            },
        ];

        // Read scene
        let rad_file = "./test_data/room.rad";
        let mut scene = Scene::from_radiance(rad_file.to_string());
        scene.build_accelerator();
        eprintln!("Ready to calc!... # Surface = {}", scene.triangles.len());

        // Initialize DC Factory
        let factory = DCFactory {
            max_depth: 4,
            n_ambient_samples: 2700,
            reinhart: ReinhartSky::new(1),
            ..DCFactory::default()
        };

        
        let dc_matrix = factory.calc_dc(&rays, &scene);
        let dc_matrix = crate::colour_matrix::colour_matrix_to_luminance(&dc_matrix);
        crate::colour_matrix::save_matrix(
            &dc_matrix,
            std::path::Path::new("./test_data/full_dc_rust.mtx"),
        )
        .unwrap();
    }
}
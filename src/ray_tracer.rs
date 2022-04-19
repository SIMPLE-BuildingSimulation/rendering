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
use std::time::Instant;

use crate::camera::{Camera, CameraSample};
use crate::colour::Spectrum;
use crate::image::ImageBuffer;
use crate::material::Material;
use crate::rand::*;
use crate::ray::Ray;
use crate::scene::{Object, Scene};
use crate::Float;
use geometry3d::intersection::SurfaceSide;
use geometry3d::{Point3D, Ray3D, Vector3D};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub struct RayTracerHelper {
    rays: Vec<Ray>,
    nodes: Vec<usize>,
}

impl std::default::Default for RayTracerHelper {
    fn default() -> Self {
        Self {
            // rays: Vec::with_capacity(10),
            rays: vec![Ray::default(); 10], //Vec::with_capacity(10),
            nodes: Vec::with_capacity(64),
        }
    }
}

pub struct RayTracer {
    pub max_depth: usize,
    pub n_shadow_samples: usize,
    pub n_ambient_samples: usize,

    pub limit_weight: Float,
    pub count_specular_bounce: Float,
}

impl Default for RayTracer {
    fn default() -> Self {
        Self {
            max_depth: 2,
            n_shadow_samples: 10,
            n_ambient_samples: 10,

            limit_weight: 1e-3,
            count_specular_bounce: 0.3,
        }
    }
}

impl RayTracer {
    /// Recursively traces a ray
    pub fn trace_ray(
        &self,
        rng: &mut RandGen,
        scene: &Scene,
        ray: &mut Ray,
        current_depth: usize,
        current_value: Float,
        aux: &mut RayTracerHelper,
    ) -> (Spectrum, Float) {
        let one_over_ambient_samples = 1. / self.n_ambient_samples as Float;

        // If hits an object
        // Store refraction index???

        if let Some(triangle_index) = scene.cast_ray(ray, &mut aux.nodes) {            
            // THIS HAS MODIFIED THE INTERACTION.            
            let material = match ray.interaction.geometry_shading.side {
                SurfaceSide::Front => &scene.materials[scene.front_material_indexes[triangle_index]],
                SurfaceSide::Back => &scene.materials[scene.back_material_indexes[triangle_index]],
                SurfaceSide::NonApplicable => {
                    // Hit parallel to the surface...
                    return (Spectrum::black(), 0.0);
                }
            };

            // let intersection_pt = ray.geometry.project(t);
            let intersection_pt = ray.interaction.point;

            // for now, emmiting materials don't reflect... but they
            // are visible when viewed directly from the camera
            if material.emits_light() {
                let light_pdf = crate::triangle::triangle_solid_angle_pdf(&scene.triangles[triangle_index], intersection_pt, ray.interaction.geometry_shading.normal, &ray.geometry);
                return (material.colour(), light_pdf);                
            }

            // Limit bounces
            if current_depth > self.max_depth {
                return (Spectrum::black(), 0.0);
            }

            // Get basic information on the intersection
            let u = ray.interaction.geometry_shading.u;
            let v = ray.interaction.geometry_shading.v;

            let normal = ray.interaction.geometry_shading.normal;
            // let n0 = scene.normals[triangle_index].0;
            // let n1 = scene.normals[triangle_index].1;
            // let n2 = scene.normals[triangle_index].2;
            // let mut normal = (n0*u + n1*v + n2*(1.-u-v)).get_normalized();
            // if normal*ray.interaction.geometry_shading.normal < 0.0{
            //     normal *= -1.
            // }

            let e1 = normal.get_perpendicular().unwrap();
            // let e1 = ray.interaction.geometry_shading.dpdu.get_normalized();
            let e2 = normal.cross(e1).get_normalized();

            // Check
            debug_assert!((1.0 - normal.length()).abs() < 1e-5);
            debug_assert!((1.0 - e1.length()).abs() < 1e-5);
            debug_assert!((1.0 - e2.length()).abs() < 1e-5);
            
            let mut wt = current_value;

            // Handle specular materials... we have 1 or 2 rays... spawn those.
            if material.specular_only() {
                let mut specular_li = Spectrum::black();
                let paths = material.get_possible_paths(&normal, &intersection_pt, ray);
                // let mut n = 0;
                for (new_ray, bsdf_value, _ray_weight) in paths.iter().flatten() {
                    // n += 1;
                    let mut new_ray = *new_ray;

                    let new_ray_dir = new_ray.geometry.direction;
                    let cos_theta = (normal * new_ray_dir).abs();
                    let new_value = wt * bsdf_value * cos_theta;
                    // russian roulette
                    // if self.limit_weight > 0. && new_value < self.limit_weight {
                    // }
                    // avoid infinite interior bouncing
                    let new_depth = {
                        let q: Float = rng.gen();
                        if q < self.count_specular_bounce {
                            current_depth + 1
                        } else {
                            current_depth
                        }
                    };
                    // let new_depth = current_depth;

                    let (li, _light_pdf) =
                        self.trace_ray(rng, scene, &mut new_ray, new_depth, new_value, aux);

                    let color = material.colour();
                    // let total_samples = n + n_lights * self.n_shadow_samples;
                    // let bsdf_c = 1.;//n as Float / total_samples as Float;
                    specular_li += (li * cos_theta * *bsdf_value) * (color);// * *_ray_weight;
                                                                             // / ( bsdf_c * bsdf_value );
                }
                // return Some(specular_li + local)
                return (specular_li, 0.0);
            }

            
            let n_ambient_samples = if self.max_depth == 0 {
                0 // No ambient samples required
            } else if current_depth == 0 {
                self.n_ambient_samples
            } else {
                /* Adapted From Radiance's samp_hemi() at src/rt/ambcomp.c */

                let d = 0.8 * current_value /*intens(rcol)*/* current_value * one_over_ambient_samples
                    / self.limit_weight;
                if wt > d {
                    wt = d;
                }
                let n = ((self.n_ambient_samples as Float * wt).sqrt() + 0.5).round() as usize;

                const MIN_AMBS: usize = 1;
                if n < MIN_AMBS {
                    MIN_AMBS
                } else {
                    n
                }
            };

            // Calculate the number of direct samples

            let n_shadow_samples = if current_depth == 0 {
                self.n_shadow_samples
            } else {
                // 1
                self.n_shadow_samples
            };

            /* DIRECT LIGHT */
            // let local = Spectrum::black();
            let local = self.get_local_illumination(
                scene,
                material,
                ray,
                intersection_pt,
                normal,
                e1,
                e2,
                rng,
                n_ambient_samples,
                n_shadow_samples,
                &mut aux.nodes,
            );

            /* INDIRECT */
            let global = self.get_global_illumination(
                scene,
                n_ambient_samples,
                n_shadow_samples,
                current_depth,
                material,
                normal,
                e1,
                e2,
                ray,
                intersection_pt,
                wt,
                rng,
                aux,
            );

            // global /= n as Float;
            // global /= total_samples as Float;

            // return
            ((local + global), 0.0) // /total_samples as Float , 0.0)
        } else {
            // Did not hit... so, let's check the sky
            if let Some(sky) = &scene.sky {
                let sky_brightness = sky(ray.geometry.direction);
                let colour = scene.sky_colour.unwrap_or_else(||Spectrum::gray(1.0));
                (colour * sky_brightness, 1. / 2. / crate::PI)
            } else {
                (Spectrum::black(), 0.0)
            }
        }
    }

    fn sample_light_array(
        &self,
        scene: &Scene,
        material: &Material,
        ray: &Ray,
        intersection_pt: Point3D,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        rng: &mut RandGen,
        n_ambient_samples: usize,
        n_shadow_samples: usize,
        lights: &[Object],
        node_aux: &mut Vec<usize>,
    ) -> Spectrum {
        let mut local_illum = Spectrum::black();
        for light in lights.iter() {
            // let this_origin = this_origin + normal * 0.001;
            let mut i = 0;
            // let mut missed = 0;
            while i < n_shadow_samples {
                let direction = light.primitive.sample_direction(rng, intersection_pt);
                // let (_,direction) = light.primitive.direction( point);
                let shadow_ray = Ray3D {
                    origin: intersection_pt,
                    direction,
                };

                if let Some((light_colour, light_pdf)) =
                    sample_light(scene, light, &shadow_ray, node_aux)
                {
                    i += 1;
                    if light_pdf < 1e-18 {
                        // The light is obstructed... don't add light, but count it.
                        continue;
                    }

                    // Denominator of the Balance Heuristic... I am assuming that
                    // when one light has a pdf>0, then all the rest are Zero... is this
                    // correct?
                    let cos_theta = (normal * direction).abs();
                    let vout = shadow_ray.direction * -1.;

                    let mat_bsdf_value = material.eval_bsdf(normal, e1, e2, ray, vout);
                    // let denominator = mat_bsdf_value * bsdf_c + light_pdf * light_c; //light_pdf;//
                    let denominator = light_pdf * n_shadow_samples as Float
                        + mat_bsdf_value.radiance() * n_ambient_samples as Float; //light_pdf;//
                    let fx = (light_colour * cos_theta) * (mat_bsdf_value);

                    // Return... light sources have a pdf equal to their 1/Omega (i.e. their size)
                    local_illum += fx / denominator;
                } else {
                    #[cfg(debug_assertions)]
                    {                        
                        eprintln!("Missed Light... primitive '{}' (i = {})", light.primitive.id(), i)
                    }
                }
                // ... missed light. Try again
            } // end of iterating samples
        } // end of iterating lights

        local_illum
    }

    /// Calculates the luminance produced by the direct sources in the
    /// scene
    fn get_local_illumination(
        &self,
        scene: &Scene,
        material: &Material, //&impl Material,
        ray: &Ray,
        mut intersection_pt: Point3D,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        rng: &mut RandGen,
        n_ambient_samples: usize,
        n_shadow_samples: usize,
        node_aux: &mut Vec<usize>,
    ) -> Spectrum {
        // prevent self-shading
        intersection_pt += normal * 0.001;
        let close = self.sample_light_array(
            scene,
            material,
            ray,
            intersection_pt,
            normal,
            e1,
            e2,
            rng,
            n_ambient_samples,
            n_shadow_samples,
            &scene.lights,
            node_aux,
        );
        let distant = self.sample_light_array(
            scene,
            material,
            ray,
            intersection_pt,
            normal,
            e1,
            e2,
            rng,
            n_ambient_samples,
            n_shadow_samples,
            &scene.distant_lights,
            node_aux,
        );

        // return
        close + distant
    }

    fn get_global_illumination(
        &self,
        scene: &Scene,
        n_ambient_samples: usize,
        n_shadow_samples: usize,
        current_depth: usize,
        material: &Material,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        ray: &mut Ray,
        intersection_pt: Point3D,
        wt: Float,
        rng: &mut RandGen,
        aux: &mut RayTracerHelper,
    ) -> Spectrum {
        let mut global = Spectrum::black();
        let depth = current_depth; //ray.depth;
        aux.rays[depth] = *ray;

        for _ in 0..n_ambient_samples {
            // Choose a direction.
            let (bsdf_value, pdf) = material.sample_bsdf(normal, e1, e2, intersection_pt, ray, rng);
            let new_ray_dir = ray.geometry.direction;
            debug_assert!(
                (1. - new_ray_dir.length()).abs() < 1e-5,
                "Length is {}",
                new_ray_dir.length()
            );

            // increase depth
            let new_depth = current_depth + 1;
            let cos_theta = (normal * new_ray_dir).abs();
            let new_value = bsdf_value.radiance() * wt * cos_theta / pdf;

            // russian roulette
            // if self.limit_weight > 0. && new_value < self.limit_weight {
            //     let q: Float = rng.gen();
            //     if q > new_value / self.limit_weight {
            //         continue;
            //     }
            // }

            let (li, light_pdf) = self.trace_ray(rng, scene, ray, new_depth, new_value, aux);

            let fx = (li * cos_theta) * bsdf_value;
            let denominator = bsdf_value.radiance() * n_ambient_samples as Float
                + n_shadow_samples as Float * light_pdf;

            global += fx / denominator;

            // restore ray, because it was modified by trace_ray executions
            *ray = aux.rays[depth];
        }
        // return
        global
    }

    pub fn render(self, scene: &Scene, camera: &dyn Camera) -> ImageBuffer {
        let (width, height) = camera.film_resolution();

        let total_pixels = width * height;
        let mut pixels = vec![
            Spectrum {
                red: 0.,
                green: 0.,
                blue: 0.
            };
            total_pixels
        ];

        let n_threads = 8;
        let chunk_len = total_pixels / n_threads;
        let i: Vec<&mut [Spectrum]> = pixels.chunks_mut(chunk_len).collect();

        #[cfg(not(feature = "parallel"))]
        let i = i.into_iter();

        #[cfg(feature = "parallel")]
        let i = i.into_par_iter();

        let now = Instant::now();
        // progress indicators
        let last_progress = std::sync::Arc::new(std::sync::Mutex::new(0.0));
        let counter = std::sync::Arc::new(std::sync::Mutex::new(0));

        let _ = &i.enumerate().for_each(|(first_p, chunk)| {
            let mut pindex = first_p * chunk_len;
            let mut aux = RayTracerHelper::default();
            let mut rng = get_rng();

            for pixel in chunk {
                let y = (pindex as Float / width as Float).floor() as usize;
                let x = pindex - y * width;
                let (mut ray, weight) = camera.gen_ray(&CameraSample { p_film: (x, y) });
                
                let (v, _) = self.trace_ray(&mut rng, scene, &mut ray, 0, weight, &mut aux);
                *pixel = v;

                // report
                let mut c = counter.lock().unwrap();
                *c += 1;

                let mut lp = last_progress.lock().unwrap();
                let progress = (100. * *c as Float / total_pixels as Float).round() as Float;
                if (*lp - progress.floor()) < 0.1 && (progress - *lp).abs() > 1. {
                    *lp = progress;
                    println!("... Done {:.0}%", progress);
                }

                pindex += 1;
                
            }
        });

        println!("Scene took {} seconds to render", now.elapsed().as_secs());

        // return
        ImageBuffer::from_pixels(width, height, pixels)
    }
}

/// Sends a `shadow_ray` towards a `light`. Returns `None` if the ray misses
/// the light, returns `Some(Black, 0)` if obstructed; returns `Some(Color, pdf)`
/// if the light is hit.
#[inline(never)]
pub fn sample_light(
    scene: &Scene,
    light: &Object,
    shadow_ray: &Ray3D,
    node_aux: &mut Vec<usize>,
) -> Option<(Spectrum, Float)> {
    let light_direction = shadow_ray.direction;
    let origin = shadow_ray.origin;

    // Expect direction to be normalized
    debug_assert!((1. - light_direction.length()).abs() < 0.0001);

    let info = light.primitive.intersect(shadow_ray)?;

    let light_distance_squared = (origin - info.p).length_squared();

    // If the light is not visible (this should not consider
    // transparent surfaces, yet.)
    if !scene.unobstructed_distance(shadow_ray, light_distance_squared, node_aux) {        
        return Some((Spectrum::black(), 0.0));
    }

    let light_material = match info.side {
        SurfaceSide::Front => &scene.materials[light.front_material_index],
        SurfaceSide::Back => &scene.materials[light.back_material_index],
        SurfaceSide::NonApplicable => {
            // Hit parallel to the surface
            return Some((Spectrum::black(), 0.0));
        }
    };
    // let light_material = &scene.materials[light.front_material_index];

    let light_colour = light_material.colour();
    let light_pdf = light.primitive.solid_angle_pdf(&info, &shadow_ray);

    // let light_pdf = 1. / light.primitive.omega(origin);

    // return
    Some((light_colour, light_pdf))
}

#[cfg(test)]
mod tests {
    // use super::*;


}

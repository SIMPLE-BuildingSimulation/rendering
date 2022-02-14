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

use crate::Float;
use crate::camera::{Camera, CameraSample};
use crate::image::ImageBuffer;
use crate::scene::{Scene, Object};
use geometry3d::{Ray3D, Point3D, Vector3D};
use crate::colour::Spectrum;
use geometry3d::intersection::SurfaceSide;
use crate::ray::Ray;
use crate::interaction::Interaction;
use crate::rand::*;
use crate::material::Material;

#[cfg(feature="parallel")]
use rayon::prelude::*;


pub struct RayTracer {
    pub max_depth: usize,
    pub n_shadow_samples: usize,
    pub n_ambient_samples: usize, 
    
    
    pub limit_weight: Float,
    pub limit_reflections: usize,
}

impl Default for RayTracer{
    fn default()->Self{
        Self{
            max_depth: 2,
            n_shadow_samples: 10,
            n_ambient_samples: 10,      
            
            
            limit_weight: 1e-3,
            limit_reflections: 0,
        }
    }
}

impl RayTracer {

     /// Recursively traces a ray
     pub fn trace_ray(&self, rng: &mut RandGen, scene: &Scene, ray: Ray, current_depth: usize, current_value: Float) -> (Spectrum, Float) {
        
        let one_over_ambient_samples = 1. / self.n_ambient_samples as Float;

        


        // If hits an object 
        if let Some((t, Interaction::Surface(data))) = scene.cast_ray(&ray) {

            let object = &scene.objects[data.prim_index];            
            let material = match data.geometry_shading.side {
                SurfaceSide::Front => {
                    &scene.materials[object.front_material_index]
                },
                SurfaceSide::Back =>{
                    &scene.materials[object.back_material_index]
                },
                SurfaceSide::NonApplicable => {
                    // Hit parallel to the surface...                     
                    return (Spectrum::black(),0.0)
                }                    
            };
            
            
            let intersection_pt = ray.geometry.project(t);            

            // for now, emmiting materials don't reflect... but they 
            // are visible when viewed directly from the camera
            if material.emits_light() {
                // if current_depth == 0 {
                    let light_pdf = 1./object.primitive.omega(intersection_pt);
                    return (material.colour(), light_pdf)
                    // return Some(Spectrum::gray(1.))
                // }else{
                //     return None; 
                    // return Some(Spectrum::black());
                // }
            }

            // Get basic information on the intersection
            
            let normal = data.geometry_shading.normal;
            let e1 = data.geometry_shading.dpdu.get_normalized();
            let e2 = normal.cross(e1);//.get_normalized();

                        
            // Check
            debug_assert!((1. - normal.length()).abs() < 1e-5);
            debug_assert!((1.0 - e1.length()).abs() < 1e-5);
            debug_assert!((1.0 - e2.length()).abs() < 1e-5);
            
            
            
                             
            // Calculate the number of ambient samples
            let mut wt = current_value;
            let n = if self.max_depth == 0 { 
                0 // No ambient samples required
            } else if current_depth == 0 {
                self.n_ambient_samples
            } else  {

                /* Adapted From Radiance's samp_hemi() at src/rt/ambcomp.c */                        
                
                let d = 0.8 * current_value /*intens(rcol)*/* current_value * one_over_ambient_samples / self.limit_weight;
                if wt > d {
                    wt = d;
                }
                let n = ((self.n_ambient_samples as Float * wt).sqrt() + 0.5).round() as usize;                    
                const MIN_AMBS : usize = 1;
                if n < MIN_AMBS {
                    MIN_AMBS
                }else{
                    n
                }            
            };
            
             


            // Handle specular materials... we have 1 or 2 rays... spawn those.            
            if material.specular_only(){
                let mut specular_li = Spectrum::black();
                let paths = material.get_possible_paths(normal, intersection_pt, ray);                
                // let mut n = 0;
                for (new_ray, bsdf_value, weight) in paths.iter().flatten(){

                    // n += 1;
                
                    let new_ray_dir = new_ray.geometry.direction;                    
                    let cos_theta = (normal * new_ray_dir).abs();
                    let new_value = wt * bsdf_value * cos_theta;
                    // russian roulette
                    if self.limit_weight > 0. && new_value < self.limit_weight {                                                        
                        let q : Float = rng.gen();
                        if q > new_value/self.limit_weight {                                
                            return (Spectrum::black(), 0.0);
                        }
                    }
                    
                    let (li, _light_pdf) = self.trace_ray(rng, scene, *new_ray, current_depth, new_value);
                        
                    let color = material.colour();
                    // let total_samples = n + n_lights * self.n_shadow_samples;
                    // let bsdf_c = 1.;//n as Float / total_samples as Float;
                    specular_li += (li * cos_theta) * (color ) * *weight;// / ( bsdf_c * bsdf_value );                        
                    
                } 
                // return Some(specular_li + local)
                return (specular_li, 0.0 )
            }         
            
            

            /* DIRECT LIGHT */
            let local = self.get_local_illumination(
                scene,
                material,
                ray,
                intersection_pt,
                normal,
                e1,e2,
                rng,
                n
            );
        
            
            // Limit bounces        
            if current_depth > self.max_depth {            
                return (local, 0.0)                
            }
            /* INDIRECT */
             
            let mut global = Spectrum::black();                    
            
            

            for _ in 0..n {
                // Choose a direction.                                
                let (new_ray, bsdf_value,  _is_specular) = material.sample_bsdf(normal, e1, e2, intersection_pt, ray, rng);                                            
                let new_ray_dir = new_ray.geometry.direction;
                debug_assert!((1. - new_ray_dir.length()).abs() < 1e-5, "Length is {}",new_ray_dir.length() );

                // increase depth 
                let mut new_depth = current_depth;// + 1;
                // if !is_specular {
                    new_depth += 1;
                // }
                let cos_theta = (normal * new_ray_dir).abs();
                let new_value = wt * bsdf_value * cos_theta;
                
                                
                // russian roulette
                if self.limit_weight > 0. && new_value < self.limit_weight {                                                        
                    let q : Float = rng.gen();
                    if q > new_value/self.limit_weight {                                
                        return (Spectrum::black(), 0.0);
                    }
                }
                
                let color = material.colour();

                let (li, light_pdf) =  self.trace_ray(rng, scene, new_ray, new_depth, new_value);                
                
                let fx =  (li * cos_theta) * (color * bsdf_value);// * n as Float;
                // let denominator = bsdf_value;// * bsdf_c;
                let denominator = bsdf_value * n as Float + self.n_shadow_samples  as Float * light_pdf; 

                // add contribution
                global += fx / denominator;
            }                    
            
            // global /= n as Float; 
            // global /= total_samples as Float;
                        
            // return
            ( (local + global), 0.0) // /total_samples as Float , 0.0)

        } else {
            // Did not hit... how about distant lights?
            (Spectrum::black(), 0.0)
        }
    }

    
    /// Sends a `shadow_ray` towards a `light`. Returns `None` if the ray misses 
    /// the light, returns `Some(Black, 0)` if obstructed; returns `Some(Color, pdf)` 
    /// if the light is hit.
    fn sample_light(&self, scene: &Scene, light: &Object, shadow_ray: &Ray3D)->Option<(Spectrum, Float)>{
        
        let light_direction = shadow_ray.direction;
        let origin = shadow_ray.origin;
        
        debug_assert!(scene.materials[light.front_material_index].emits_direct_light());

        // Expect direction to be normalized
        debug_assert!((1. - light_direction.length()).abs() < 0.0001);


        let intersection_info = match light.primitive.intersect(shadow_ray) {
            Some(info) => info,
            None => {
                // eprintln!("... Missed light...");
                return None;//(Spectrum::black(),0.0)
            }
        };


        let light_distance = (origin - intersection_info.p).length();

        // If the light is not visible (this does not consider 
        // transparent surfaces, yet.)
        if !scene.unobstructed_distance(shadow_ray, light_distance) {                        
            return Some((Spectrum::black(), 0.0))
        } 
                        
        let light_material = match intersection_info.side {
            SurfaceSide::Front => {
                &scene.materials[light.front_material_index]
            }
            SurfaceSide::Back => {
                &scene.materials[light.back_material_index]
            },
            SurfaceSide::NonApplicable => {
                // Hit parallel to the surface
                return Some((Spectrum::black(), 0.0)) ;
            }
        };

        let light_colour = light_material.colour();   
        
        let light_pdf = 1./ light.primitive.omega(origin);
        
        // return
        Some((light_colour ,light_pdf))
                 
    }

    


    /// Calculates the luminance produced by the direct sources in the
    /// scene
    fn get_local_illumination(
        &self,        
        scene: &Scene,
        material: &Material,                
        ray: Ray,
        point: Point3D,
        normal: Vector3D,  
        e1: Vector3D,
        e2: Vector3D,   
        rng: &mut RandGen,
        n_ambient_samples: usize   
    ) -> Spectrum {        
        // prevent self-shading
        let this_origin = point + normal * 0.001;
        let mat_colour = material.colour();
        
        

        // let lights = &scene.lights;
        // let n_lights = scene.lights.len() + scene.distant_lights.len();
        // let total_samples = n_ambient_samples + n_lights * self.n_shadow_samples;
        // let bsdf_c = n_ambient_samples as Float / total_samples as Float;
        // let light_c = self.n_shadow_samples as Float / total_samples as Float;

        let sample_light_array = |lights: &[Object], rng: &mut RandGen|->Spectrum{           
            // println!("---- entering sample_light_array()");
            let mut local_illum = Spectrum::black(); 
            for light in lights.iter() {
                // let this_origin = this_origin + normal * 0.001;
                let mut i = 0;
                while i < self.n_shadow_samples {
                    let direction = light.primitive.sample_direction(rng,this_origin);
                    let shadow_ray = Ray3D {
                        origin: this_origin,
                        direction,
                    };
            
                    
                    if let Some((light_colour, light_pdf)) = self.sample_light(scene, light, &shadow_ray){
                        i += 1;
                        if light_pdf < 1e-18 {
                            // The light is obstructed.                            
                            continue
                        }

                        // println!("i = {}", i);
                        // Denominator of the Balance Heuristic... I am assuming that
                        // when one light has a pdf>0, then all the rest are Zero... is this
                        // correct?
                        let cos_theta = (normal * direction).abs();
                        let vout = shadow_ray.direction * -1.;

                        let mat_bsdf_value = material.eval_bsdf(normal, e1, e2,  ray, vout );
                        // let denominator = mat_bsdf_value * bsdf_c + light_pdf * light_c; //light_pdf;//
                        let denominator = light_pdf * self.n_shadow_samples as Float + mat_bsdf_value * n_ambient_samples as Float; //light_pdf;//
                        let fx = (light_colour * cos_theta) * (mat_colour * mat_bsdf_value);// * self.n_shadow_samples as Float;
                                                
                        // Return... light sources have a pdf equal to their 1/Omega (i.e. their size)
                        local_illum += fx / denominator ;
                                                
                    }
                    // else, just try again
                    
                } // end of iterating samples
            } // end of iterating lights
            // local_illum / self.n_shadow_samples as Float // return
            local_illum 
        };

        let close = sample_light_array(&scene.lights, rng);
        let distant = sample_light_array(&scene.distant_lights, rng);        
        
        // return
        close + distant
    }


    pub fn render(&self, scene: &Scene, camera: &Camera) -> ImageBuffer {        
        let (width, height) = camera.film_resolution();
        
        let total_pixels = width * height;

        let last_progress = std::sync::Arc::new(std::sync::Mutex::new(0.0));
        let counter = std::sync::Arc::new(std::sync::Mutex::new(0));

        #[cfg(not(feature = "parallel"))]
        let aux_iter = 0..total_pixels;//.into_iter();
        #[cfg(feature = "parallel")]
        let aux_iter = (0..total_pixels).into_par_iter();

        let pixels : Vec<Spectrum> = aux_iter.map(|pixel|{
            let y = (pixel as f32/width as f32).floor() as usize;
            let x = pixel - y*width;
            let (ray, weight) = camera.gen_ray(&CameraSample {
                p_film: (x, y),
                p_lens: (0., 0.), // we will not use this                    
            });
            
            let mut rng = get_rng();   
            let (v,_) = self.trace_ray(&mut rng, scene,ray, 0, weight);
            // report
            let mut c = counter.lock().unwrap();
            *c += 1;
            
            
            let mut lp = last_progress.lock().unwrap();
            let progress = (100. *  *c as Float/ total_pixels  as Float).round() as Float;                        
            if (*lp - progress.floor()) < 0.1 && (progress - *lp).abs() > 1. {
                *lp = progress;                
                println!("... Done {:.0}%", progress);
            }
            

            // return... should always contain value because these are primary rays
            v
        }).collect();

        // return
        ImageBuffer::from_pixels(width, height, pixels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // use geometry3d::ray3d::Ray3D;
    use geometry3d::{Vector3D, Point3D};

    use crate::camera::{Camera,View};
    use crate::film::Film;
    use std::time::Instant;

    fn compare_with_radiance(filename: String) {
        

        let mut scene = Scene::from_radiance(format!("./test_data/{}", filename));
        scene.build_accelerator();

        // Create film
        let film = Film {
            resolution: (512, 512),
        };

        // Create view
        let view = View {
            view_direction: Vector3D::new(0., 1., 0.),
            view_point: Point3D::new(0., -13., 0.),
            ..View::default()
        };

        // Create camera
        let camera = Camera::pinhole(view, film);

        let integrator = RayTracer{
            n_shadow_samples: 38,
            max_depth: 3,
            limit_weight: 0.001,
            n_ambient_samples: 129,
            .. RayTracer::default()   
        };
        let now = Instant::now();
        let buffer = integrator.render(&scene, &camera);

        println!(
            "Scene '{}' took {} seconds to render",
            filename,
            now.elapsed().as_secs()
        );

        buffer.save_hdre(format!("./test_data/images/self_{}.hdr", filename));
    }
 
    #[test]
    fn render_scenes() {
        return;
        compare_with_radiance("exterior_0_diffuse_plastic.rad".to_string());
        // compare_with_radiance("exterior_0_specularity.rad".to_string());
        compare_with_radiance("exterior_0_mirror.rad".to_string());
        
    }

    #[test]
    fn render_dielectric(){
        return;
        let filename = "exterior_0_dielectric.rad".to_string();
        let mut scene = Scene::from_radiance(format!("./test_data/{}", filename));
        scene.build_accelerator();

        // Create film
        let film = Film {
            resolution: (512, 512),
        };

        // // Create view
        let view = View {
            view_direction: Vector3D::new(0., 1., 0.).get_normalized(),
            view_point: Point3D::new(2., 1., 1.),
            ..View::default()
        };

        // Create camera
        let camera = Camera::pinhole(view, film);

        let integrator = RayTracer{
            n_shadow_samples: 0,
            max_depth: 3,
            limit_weight: 0.001,
            n_ambient_samples: 20,
            .. RayTracer::default()   
        };

        let now = Instant::now();
        // let buffer = integrator.render(&scene, &camera);

        let mut rng = get_rng();

        let pixel = 2000;
        let width = 512;        
        let y = (pixel as f32/width as f32).floor() as usize;
            let x = pixel - y*width;

        let (ray, weight) = camera.gen_ray(&CameraSample{
            p_film: (x,y),
            p_lens: (0., 0.)
        });

        let (i, _) = integrator.trace_ray(&mut rng, &scene, ray, 0, weight);
        println!("{}", i);

        println!(
            "Scene '{}' took {} seconds to render",
            filename,
            now.elapsed().as_secs()
        );

        // buffer.save_hdre(format!("./test_data/images/self_{}.hdr", filename));
    }

    #[test]
    fn test_render_room() {
        // return;
        let mut scene = Scene::from_radiance("./test_data/room.rad".to_string());
        
        scene.build_accelerator();

        // Create camera
        // Create film
        let film = Film {
            resolution: (512, 512),
        };

        // Create view
        let view = View {
            view_direction: Vector3D::new(0., 1., 0.).get_normalized(),
            view_point: Point3D::new(2., 1., 1.),
            ..View::default()
        };
        // Create camera
        let camera = Camera::pinhole(view, film);

        let integrator = RayTracer {
            n_ambient_samples: 220,
            n_shadow_samples: 1,
            max_depth: 3,
            .. RayTracer::default()
        };


        // let y = (pixel as f32/width as f32).floor() as usize;
        // let x = pixel - y*width;
        // let x = 256;
        // let y = 1;
        // let (ray, weight) = camera.gen_ray(&CameraSample {
        //     p_film: (x, y),
        //     p_lens: (0., 0.), // we will not use this                    
        // });
        // let mut rng = get_rng();   
        // let v = integrator.trace_ray(&mut rng, &scene, ray, 0, weight);

        let now = Instant::now();

        let buffer = integrator.render(&scene, &camera);
        println!("Room took {} seconds to render", now.elapsed().as_secs());
        buffer.save_hdre("./test_data/images/room.hdr".to_string());

        
    }




    use crate::material::{Material, PlasticMetal};
    use geometry3d::{DistantSource3D, Triangle3D, Sphere3D};
    use crate::primitive::Primitive;    
    #[test]
    fn test_2() {
        return;
        // Build scene
        let mut scene = Scene::default();

        let red = scene.push_material(Material::Plastic(PlasticMetal {
            color:Spectrum{
                red: 0.55,
                green: 0.15,
                blue: 0.15},
            specularity: 0.,
            roughness: 0.,
        }));

        let green = scene.push_material(Material::Plastic(PlasticMetal {
            color: Spectrum{
                red: 0.15,
                green: 0.15,
                blue: 0.15
            },            
            specularity: 0.,
            roughness: 0.,
        }));

        let mirror = scene.push_material(Material::Mirror( Spectrum{
            red: 0.8,
            green: 0.99,
            blue: 0.8,
        }));

        scene.push_object(
            mirror,
            mirror,
            Primitive::Sphere(Sphere3D::new(1.5, Point3D::new(0., 0., 0.5))),
        );

        scene.push_object(
            mirror,
            red,            
            Primitive::Sphere(Sphere3D::new_partial(
                1.5, 
                Point3D::new(1., -1., -1.5),
                -2., 0.2, 
                360.
            ))            
        );

        scene.push_object(
            red,
            red,
            Primitive::Sphere(Sphere3D::new(12.5, Point3D::new(0., 25., 12.5-3.)))            
        );

        scene.push_object(
            green,
            green,
            Primitive::Triangle(Triangle3D::new(
                Point3D::new(-1000., -1000., -3.),
                Point3D::new( 1000., -1000., -3.),
                Point3D::new( 1000.,  1000., -3.),                
            ).unwrap()),
        );

        scene.push_object(
            green,
            green,
            Primitive::Triangle(Triangle3D::new(
                Point3D::new( 1000.,  1000., -3.),                
                Point3D::new(-1000.,  1000., -3.),
                Point3D::new(-1000., -1000., -3.),
            ).unwrap()),
        );

        let up = scene.push_material(Material::Light(Spectrum {
            red: 10000.,
            green: 10000.,
            blue: 10000.,
        }));

        scene.push_object(
            up,
            up,
            Primitive::Source(DistantSource3D::new(
                Vector3D::new(0., 0., 1.),         // direction
                (0.5 as Float).to_radians(), // angle
            )),
        );

        scene.push_object(
            up,
            up,
            Primitive::Source(DistantSource3D::new(
                Vector3D::new(0., -1., 1.),        // direction
                (0.5 as Float).to_radians(), // angle
            )),
        );

        let lightbulb = scene.push_material(Material::Light(Spectrum {
            red: 100.,
            green: 100.,
            blue: 100.,
        }));

        scene.push_object(
            lightbulb,
            lightbulb,
            Primitive::Sphere(Sphere3D::new(1.5, Point3D::new(1., -1., 15.))),
        );

        scene.build_accelerator();

        // Create camera
        // Create film
        let film = Film {
            resolution: (512, 512),
        };

        // Create view
        let view = View {
            view_direction: Vector3D::new(0., 1., 0.),
            view_point: Point3D::new(0., -13., 0.),
            ..View::default()
        };

        // Create camera
        let camera = Camera::pinhole(view, film);

        let integrator = RayTracer{
            n_ambient_samples: 18,
            n_shadow_samples: 15,
            max_depth: 3,
            .. RayTracer::default()
        };
        let now = Instant::now();
        let buffer = integrator.render(&scene, &camera);
        println!("Scene took {} seconds to render", now.elapsed().as_secs());
        buffer.save_hdre("./test_data/images/ray_tracer.hdr".to_string());
    }

    

}

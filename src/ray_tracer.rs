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

use std::rc::Rc;
use crate::Float;
use crate::camera::{Camera, CameraSample};
use crate::image::ImageBuffer;
use crate::scene::{Scene, Object};
use geometry3d::{Ray3D, Point3D, Vector3D};
use crate::colour::Spectrum;
use geometry3d::intersect_trait::SurfaceSide;
use crate::material::Material;
use crate::ray::Ray;
use crate::interaction::Interaction;
use rand::prelude::*;

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
            
            
            limit_weight: 1e-5,
            limit_reflections: 0,
        }
    }
}

impl RayTracer {

     /// Recursively traces a ray
     pub fn trace_ray(&self, rng: &mut ThreadRng, scene: &Scene, ray: &Ray, current_depth: usize, current_value: Float) -> Spectrum {
        
        // Limit bounces        
        if current_depth > self.max_depth {
            return Spectrum::black();
        }
        // Check reflection limits... as described in RTRACE's man
        if current_value < self.limit_weight && self.limit_reflections > 0 {
            return Spectrum::black();
        }else{
            // russian roulette
            let q : Float = rng.gen();
            if q > current_value/self.limit_weight {
                return Spectrum::black();
            }
        }

        // If hits an object
        if let Some((t, interaction)) = scene.cast_ray(ray) {

            let object = interaction.object();
            match &interaction {
                Interaction::Endpoint(_)=>{panic!("Found an Endpoint while ray-tracing!")},
                Interaction::Surface(data)=>{         
                    // get the normal... can be textured.           
                    let normal = data.normal();
                    

                    debug_assert!((1.0 - normal.length()).abs() < 0.000001);

                    // let object = interaction.object();

                    let material = match data.geometry_shading.side {
                        SurfaceSide::Front => {
                            &scene.materials[object.front_material_index]
                        },
                        SurfaceSide::Back =>{
                            &scene.materials[object.back_material_index]
                        }                        
                    };
                    
                    let intersection_pt = ray.geometry.project(t);
                    
                    let ray_dir = ray.geometry.direction;

                   
                    /* SAMPLE LIGHTS */
                    let local = self.get_local_illumination(
                        scene,
                        material,
                        ray_dir,
                        intersection_pt,
                        normal                
                    );

                    /* SAMPLE BSDF */
                    let mut global = Spectrum::black();
                    // for now, emmiting materials don't reflect
                    if !material.emits_direct_light() {
                        let n_lights = scene.count_all_lights();
                        let total_samples = self.n_ambient_samples + n_lights * self.n_shadow_samples;
                        let bsdf_c = self.n_ambient_samples as Float / total_samples as Float;
                        for _ in 0..self.n_ambient_samples {
                            // Choose a direction.
                            let new_ray_dir = material.sample_bsdf(rng, ray_dir, data.geometry_shading);
                            debug_assert!((1.-new_ray_dir.length()).abs() < 0.0000001);
                            let new_ray = Ray{
                                time: ray.time,
                                geometry: Ray3D {
                                    direction : new_ray_dir,
                                    origin: intersection_pt,// + normal * 0.0001, // avoid self shading
                                }
                            };
                            let cos_theta = (normal * new_ray_dir).abs();
                            let material_pdf = material.bsdf(ray_dir, normal, new_ray_dir);
                            let new_value = material.colour().red * material_pdf * cos_theta;
                            let li = self.trace_ray(rng, scene, &new_ray, current_depth + 1, new_value);

                            let fx = (li * cos_theta) * (material.colour() * material_pdf);
                            let denominator = material_pdf * bsdf_c;

                            // add contribution
                            global += fx / denominator;
                        }
                        global /= total_samples as Float;
                    }

                    local + global

                }
            }            
        } else {
            // Did not hit.
            Spectrum::black()
        }
    }

    
    fn sample_light(&self, scene: &Scene, light: &Object, shadow_ray: &Ray3D)->(Spectrum, Float){
        
        let light_direction = shadow_ray.direction;
        
        let origin = shadow_ray.origin;

        debug_assert!(scene.materials[light.front_material_index].emits_direct_light());

        // Expect direction to be normalized
        debug_assert!((1. - light_direction.length()).abs() < 0.0001);


        let intersection_info = match light.primitive.intersect(&shadow_ray) {
            Some(info) => info,
            None => {
                eprintln!("... Missed light...");
                return (Spectrum::black(),0.0)
            }
        };

        let light_distance = (origin - intersection_info.p).length();

        // If the light is not visible
        if !scene.unobstructed_distance(&shadow_ray, light_distance) {                        
            return (Spectrum::black(), 0.0)
        } // end of unobstructed distance
        // otherwise, continue
        let side = intersection_info.side;
        

        let light_material = match side {
            SurfaceSide::Front => {
                &scene.materials[light.front_material_index]
            }
            SurfaceSide::Back => {
                &scene.materials[light.back_material_index]
            }
        };

        let light_colour = light_material.colour();        
        let light_pdf = 1. / light.primitive.omega(origin);
        // return
        (light_colour,light_pdf)
                 
    }

    


    /// Calculates the luminance produced by the direct sources in the
    /// scene
    fn get_local_illumination(
        &self,
        scene: &Scene,
        material: &Rc<dyn Material>,
        vin: Vector3D,
        point: Point3D,
        normal: Vector3D,        
    ) -> Spectrum {        
        // prevent self-shading
        let origin = point + normal * 0.0001;
        
        
        let mut ret = Spectrum::black();

        let lights = &scene.lights;
        let n_lights = lights.len();
        let total_samples = self.n_ambient_samples + n_lights * self.n_shadow_samples;
        let bsdf_c = self.n_ambient_samples as Float / total_samples as Float;
        let light_c = self.n_shadow_samples as Float / total_samples as Float;

        let mut sample_light_array = |lights: &[Rc<Object>]|{            
            for light in lights.iter() {
                let sampler = light.primitive.direction_sampler(origin, self.n_shadow_samples);
                for light_direction in sampler {            
                    let shadow_ray = Ray3D {
                        origin : origin + normal*0.000000001,
                        direction: light_direction,
                    };
            
                    let cos_theta = (normal * light_direction).abs();
    
                    let (light_colour, light_pdf) = self.sample_light(scene, light, &shadow_ray);            
                    if light_pdf.abs() < 100.*Float::EPSILON{                        
                        continue;
                    }
                    // Denominator of the Balance Heuristic... I am assuming that
                    // when one light has a pdf>0, then all the rest are Zero... is this
                    // correct?
                    let material_pdf = material.bsdf(shadow_ray.direction * -1., normal, vin );
                    let denominator = material_pdf * bsdf_c + light_pdf * light_c;
                    let fx = (light_colour * cos_theta) * (material.colour() * material_pdf);
    
                    // Return... light sources have a pdf equal to their 1/Omega (i.e. their size)
                    ret += fx / denominator;
                } // end of iterating samples
            } // end of iterating lights
        };

        sample_light_array(&scene.lights);
        sample_light_array(&scene.distant_lights);        

        // return
        ret / total_samples as Float
    }


    pub fn render(&self,scene: &Scene, camera: &dyn Camera) -> ImageBuffer {        
        let (width, height) = camera.film_resolution();
        let mut buffer = ImageBuffer::new(width, height);
        let total_pixels = width * height;

        let mut last_progress: Float = 0.0;
        let mut i = 0;
        for y in 0..height {
            for x in 0..width {
                let (ray, weight) = camera.gen_ray(&CameraSample {
                    p_film: (x, y),
                    p_lens: (0., 0.), // we will not use this
                    time: 1.,         // we will not use
                });
                let mut rng = rand::thread_rng();
                buffer[(x, y)] = self.trace_ray(&mut rng, scene,&ray, 0, 1.) * weight;
                // report
                let progress = (100 * i) as Float / total_pixels as Float;
                if (progress - progress.floor()) < 0.1 && (progress - last_progress).abs() > 1. {
                    last_progress = progress;
                    println!("... Done {:.0}%", progress);
                }
                // increase counter
                i += 1;
            }
        }

        buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // use std::process::Command;

    // use geometry3d::ray3d::Ray3D;
    use geometry3d::{Vector3D, Point3D};

    use crate::camera::{PinholeCam, View};
    use crate::film::Film;
    use std::time::Instant;

    fn compare_with_radiance(filename: String) {
        let now = Instant::now();

        let scene = Scene::from_radiance(format!("./test_data/{}", filename));

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
        let camera = PinholeCam::new(view, film);

        let integrator = RayTracer{
            n_shadow_samples: 3,
            limit_weight: 0.001,
            .. RayTracer::default()   
        };

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
        // return;
        compare_with_radiance("exterior_0_diffuse_plastic.rad".to_string());
        // compare_with_radiance("exterior_0_specularity.rad".to_string());
        compare_with_radiance("exterior_0_mirror.rad".to_string());
        // compare_with_radiance("exterior_0_dielectric.rad".to_string());
    }

    use crate::material::{Light, Mirror, Plastic};
    use geometry3d::{DistantSource3D, Triangle3D, Sphere3D};
    use std::rc::Rc;

    #[test]
    fn test_2() {
        // return;
        // Build scene
        let mut scene = Scene::default();

        let red = scene.push_material(Rc::new(Plastic {
            red: 0.55,
            green: 0.15,
            blue: 0.15,
            specularity: 0.,
            roughness: 0.,
        }));

        let green = scene.push_material(Rc::new(Plastic {
            red: 0.15,
            green: 0.15,
            blue: 0.15,
            specularity: 0.,
            roughness: 0.,
        }));

        let mirror = scene.push_material(Rc::new(Mirror {
            red: 0.8,
            green: 0.99,
            blue: 0.8,
        }));

        scene.push_object(
            mirror,
            mirror,
            Box::new(Sphere3D::new(1.5, Point3D::new(0., 0., 0.5))),
        );

        scene.push_object(
            mirror,
            red,            
            Box::new(Sphere3D::new_partial(
                1.5, 
                Point3D::new(1., -1., -1.5),
                -2., 0.2, 
                360.
            ))            
        );

        scene.push_object(
            red,
            red,
            Box::new(Sphere3D::new(12.5, Point3D::new(0., 25., 12.5-3.)))            
        );

        scene.push_object(
            green,
            green,
            Box::new(Triangle3D::new(
                Point3D::new(-1000., -1000., -3.),
                Point3D::new( 1000., -1000., -3.),
                Point3D::new( 1000.,  1000., -3.),                
            ).unwrap()),
        );

        scene.push_object(
            green,
            green,
            Box::new(Triangle3D::new(
                Point3D::new( 1000.,  1000., -3.),                
                Point3D::new(-1000.,  1000., -3.),
                Point3D::new(-1000., -1000., -3.),
            ).unwrap()),
        );

        let up = scene.push_material(Rc::new(Light {
            red: 10000.,
            green: 10000.,
            blue: 10000.,
        }));

        scene.push_object(
            up,
            up,
            Box::new(DistantSource3D::new(
                Vector3D::new(0., 0., 1.),         // direction
                (0.5 as Float).to_radians(), // angle
            )),
        );

        scene.push_object(
            up,
            up,
            Box::new(DistantSource3D::new(
                Vector3D::new(0., -1., 1.),        // direction
                (0.5 as Float).to_radians(), // angle
            )),
        );

        let lightbulb = scene.push_material(Rc::new(Light {
            red: 100.,
            green: 100.,
            blue: 100.,
        }));

        scene.push_object(
            lightbulb,
            lightbulb,
            Box::new(Sphere3D::new(1.5, Point3D::new(1., -1., 15.))),
        );

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
        let camera = PinholeCam::new(view, film);

        let integrator = RayTracer{
            n_ambient_samples: 3,
            n_shadow_samples: 1,
            max_depth: 2,
            .. RayTracer::default()
        };

        let buffer = integrator.render(&scene, &camera);

        buffer.save_hdre("./test_data/images/ray_tracer.hdr".to_string());
    }

    #[test]
    fn test_ray_tracer_shadow_ray(){
         // Create a scene
         let mut scene = Scene::default();
        
         // Add a Light
         let lightbulb = scene.push_material(Rc::new(Light {
             red: 100.,
             green: 100.,
             blue: 100.,
         }));
 
         scene.push_object(
             lightbulb,
             lightbulb,
             Box::new(Sphere3D::new(1.5, Point3D::new(0., 0., 15.))),
         );
 
         // Add a material and an object
         let green = scene.push_material(Rc::new(Plastic {
             red: 0.15,
             green: 0.15,
             blue: 0.15,
             specularity: 0.,
             roughness: 0.,
         }));
 
         let centre = Point3D::new(0., 0., 0.);
         let radius = 1.;
         scene.push_object(
             green,
             green,
             Box::new(Sphere3D::new(radius, centre))            
         );
 
         // Crate this so we can test methods
         let integrator = RayTracer::default();
             
         let test_ray = |given_z_origin: Float|->Result<(),String>{
            let z_origin = given_z_origin*radius;
            let expect_hit = z_origin.abs() < radius;
            let expect_visible_light = z_origin > centre.z;
            let ray = Ray{
                time: 1.,
                geometry: Ray3D{
                   origin: Point3D::new(0., -20., z_origin),
                   direction: Vector3D::new(0., 1., 0.),
                }
            };
   
            if let Some((t,interaction)) = scene.cast_ray(&ray){
                if !expect_hit {
                    return Err(format!("Z = {} | We were NOT expecting any hit",given_z_origin))
                }
                let object = interaction.object();
                if let Interaction::Surface(data) = &interaction {
                    let phit = data.point;
                    let direction = (phit - ray.geometry.origin).get_normalized();
                    assert_eq!(direction,Vector3D::new(0., 1., 0.));

                    let normal = data.normal();
                    // normal.z must have the same sign as (z_origin - centre.z)
                    if normal.z.signum()*(z_origin - centre.z).signum() == -1.{
                        return Err(format!("Z = {} | The sign of normal.z is {}, while the sign of (z_origin - centre.z) is {}", given_z_origin, normal.z.signum(), (z_origin - centre.z).signum()))
                    }
                    debug_assert!((1.0 - normal.length()).abs() < 0.000001);


                    let material = match data.geometry_shading.side {
                        SurfaceSide::Front => {
                            &scene.materials[object.front_material_index]
                        },
                        SurfaceSide::Back =>{
                            return Err(format!("Z = {} | Expecting intersection to be at the Front of the sphere", given_z_origin))
                        }                        
                    };

                    let intersection_pt = ray.geometry.project(t);
                    
                    let ray_dir = ray.geometry.direction;

                    let local_light = integrator.get_local_illumination(
                        &scene,
                        material,
                        ray_dir,
                        intersection_pt,
                        normal                
                    );
                    if local_light.is_black() && expect_visible_light{
                        return Err(format!("Z = {} | Expecting visible light... found {:?}", given_z_origin,local_light));
                    }else if !local_light.is_black() && !expect_visible_light{
                        return Err(format!("Z = {} | NOT Expecting visible light... found {:?}",given_z_origin, local_light));
                    }
                    return Ok(())

                } else{
                    panic!("Z = {} | Unexpected kind of interaction : {:?}", given_z_origin,interaction)
                }

            }else{
                if expect_hit {
                    return Err(format!("Z = {} | We were expecting a hit...", given_z_origin))
                }                
            }

            Ok(())

         };// end of closure

        test_ray(0.1).unwrap();
        test_ray(0.2).unwrap();
        test_ray(0.3).unwrap();
        test_ray(-0.1).unwrap();
        test_ray(-0.2).unwrap();
        test_ray(-0.3).unwrap();
 
    }
        
}

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
use crate::interaction::{Interaction};
use geometry3d::{Vector3D, Ray3D, Point3D};
use geometry3d::intersection::SurfaceSide;
use crate::colour::Spectrum;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::rand::*;
use crate::scene::Object;
use crate::ray_tracer::sample_light;
use crate::material::Material;


pub struct Path<'a> {
    nodes: Vec<PathNode<'a>>,
    pub start: Point3D,
    pub primary_dir: Option<Vector3D>,
} 

impl <'a>Path<'a> {

    pub fn new(start: Point3D)->Self{
        Self{
            nodes: Vec::new(),
            start,
            primary_dir: None,
        }
    }

    

    pub fn with_capacity(start: Point3D, capacity: usize)->Self{
        Self{
            nodes: Vec::with_capacity(capacity),
            start,
            primary_dir: None,
        }
    }

    

    pub fn new_from_random_walk(primary_ray: &Ray, scene: &'a Scene,  n_nodes: usize, rng: &mut RandGen)->Self{
        
        let mut ret = Self::with_capacity(primary_ray.geometry.origin, n_nodes * 2); // leave some room for future expansions
        let mut ray = *primary_ray;
        ret.primary_dir = Some(ray.geometry.direction);
        for _ in 0..n_nodes {

            if PathNode::extend_path(&mut ret, scene, &ray, rng){
                // Create new ray.
                let node = ret.nodes.last().unwrap();
                let material = node.material;
                let normal = node.normal;
                let e1 = node.e1;
                let e2 = node.e2;
                let point = node.point;

                let (new_ray, _, _) = material.sample_bsdf(normal, e1, e2, point, ray, rng);
                ray = new_ray;

            }else{
                // Did not hit anything... no point in keeping going.
                return ret;
            }

        }

        // return
        ret
    }

    fn push(&mut self, node: PathNode<'a>){
        if self.nodes.is_empty(){
            // Fill primary_dir
            let dir = (node.point - self.start).get_normalized();
            self.primary_dir = Some(dir);
        }
        self.nodes.push(node)
    }



    /// Walks from start to end, adding the contribution of the different 
    /// nodes.
    pub fn eval_from_node(&self, i: usize, scene: &Scene) -> Spectrum {        
        let mut ret = Spectrum::black();
        if self.nodes.is_empty(){
            return ret;
        }
        
        let prev_pt = if i == 0 { 
            self.start 
        } else {
            self.nodes[i-1].point
        };
        
        
        // Add local
        let node = &self.nodes[i];
        
        let vin = (node.point - prev_pt).get_normalized();
        ret += node.eval(vin);


        // Add next node
        if let Some(next_node) = self.nodes.get(i+1){
            
            

            let vout = next_node.point - node.point;
            let distance_squared = vout.length_squared();          
            let vout = vout.get_normalized();
            // Ray frim this node to the next one
            let shadow_ray = Ray3D {
                origin: node.point,
                direction: vout
            };

            // if next node is obstructed... then don't bother.            
            if !scene.unobstructed_distance(&shadow_ray, distance_squared) {
                return ret
            }

            // Ray from prev_point to this node
            let ray = Ray{
                geometry: Ray3D {
                    origin: prev_pt,
                    direction: vin
                },
                refraction_index: 1.,
            };

            
            let bsdf = node.material.eval_bsdf(node.normal, node.e1, node.e2, ray, vout);
            let cos_theta = (node.normal * vout).abs();

            ret += node.material.colour() * bsdf * cos_theta * self.eval_from_node(i+1, scene);
        }
        
        // return 
        ret
    }
}






struct PathNode<'a> {    
    normal : Vector3D,
    e1 : Vector3D,
    e2 : Vector3D,
    point: Point3D,

    /// A vector containing the radiance and the direction of direct lighting 
    /// reaching a point
    local_illuminance: Vec<(Vector3D, Spectrum)>,

    material: &'a Material
}

impl <'a>PathNode<'a> {

    /// Adds one level to the Path by sending a `ray` through the `scene`.
    /// 
    /// If the object that the ray hits has a fully specular material—e.g., Dielectric—
    /// then another ray will be sent to reflect the quasi-deterministic nature of 
    /// specular reflections.
    #[must_use]
    pub fn extend_path(path: &mut Path<'a>, scene: &'a Scene, ray: &Ray, rng: &mut RandGen)-> bool {
        if let Some(Interaction::Surface(data)) = scene.cast_ray(&ray) {
            let object = &scene.objects[data.prim_index];
            let material = match data.geometry_shading.side {
                SurfaceSide::Front => &scene.materials[object.front_material_index],
                SurfaceSide::Back => &scene.materials[object.back_material_index],
                SurfaceSide::NonApplicable => {
                    // Hit parallel to the surface...?
                    unreachable!();
                }
            };

            if material.emits_light() {
                todo!()
            }

            if material.specular_only() {
                todo!()
            }

            // let point = data.point;

            // GET LOCAL ILLUMINATION
            let normal = data.geometry_shading.normal;
            let e1 = data.geometry_shading.dpdu.get_normalized();
            let e2 = normal.cross(e1); //.get_normalized();
            let point = data.point;
            
            let n_shadow_samples : usize = 1;
            let n_lights = scene.count_all_lights();
            let mut local_illuminance: Vec<(Vector3D, Spectrum)> = Vec::with_capacity(n_lights);
            for light in &scene.lights {
                local_illuminance.push(get_local_illumination(
                    point,
                    normal,
                    n_shadow_samples,
                    light,
                    rng,
                    scene,
                    material
                ))
            }

            for light in &scene.distant_lights {
                local_illuminance.push(get_local_illumination(
                    point,
                    normal,
                    n_shadow_samples,
                    light,
                    rng,
                    scene,
                    material
                ))
            }

            // Build and push            
            path.push(PathNode {
                local_illuminance,
                normal,
                e1,
                e2,
                point,
                material,
            });
            
        }else{
            return false
        }
        // return
        true
    }


    /// Evaluates the local illumination of a node, as seen
    /// from a certain point.
    /// 
    /// `vout` goes from the intersection point to the point of view
    #[must_use]
    pub fn eval(&self, vin: Vector3D) -> Spectrum {
        debug_assert!( (1. - vin.length()).abs() < 1e-5, "length is {}", vin.length() );

        // These variables relate to 
        let normal = self.normal;
        
        let ray = Ray {
            geometry: Ray3D {
                origin: self.point,
                direction: vin
            },
            refraction_index: 1.
        };
        let mat_colour = self.material.colour();

        let mut ret = Spectrum::black();

        // Denominator of the Balance Heuristic... I am assuming that
        
        for (direction, radiance) in &self.local_illuminance{
            let direction = *direction;
            let radiance = *radiance;

            let cos_theta = (normal * direction).abs();
            // let vout = shadow_ray.direction * -1.;
            
            let mat_bsdf_value = self.material.eval_bsdf(normal, self.e1, self.e2, ray, direction*-1.);            
            ret += (radiance * cos_theta) * (mat_colour * mat_bsdf_value);                        
        }

        ret
        
    }
}





fn get_local_illumination(
    mut point: Point3D, 
    normal: Vector3D, 
    n_shadow_samples: usize, 
    light: &Object, 
    rng: &mut RandGen, 
    scene: &Scene,
    material:&Material,
)->(Vector3D, Spectrum)
{
    let mut ret_light = Spectrum::black();
    let mut average_direction = Vector3D::new(0., 0., 0.);

    let mat_colour = material.colour();

    // prevent self-shading... this assumes we are reflecting   
    point += normal * 0.001;

    let mut i = 0;
    while i < n_shadow_samples {
        let direction = light.primitive.sample_direction(rng, point);
        let shadow_ray = Ray3D {
            origin: point,
            direction,
        };
        if let Some((light_colour, light_pdf)) = sample_light(scene, light, &shadow_ray)
        {
            i += 1;
            if light_pdf < 1e-18 {
                // The light is obstructed... don't add light, but count it.
                continue;
            }
            
            
            average_direction += direction;
            ret_light += light_colour /( light_pdf * n_shadow_samples as Float );
            
        } else {
            // missed += 1;
            // eprintln!("Missed Light! {} (i = {})", missed, i)
        }
    }

    // return
    (average_direction,ret_light)

}

// primitive.intersect(ray) --> IntersectionInfo


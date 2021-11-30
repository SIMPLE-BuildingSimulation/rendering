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

// use std::rc::RefCount;
use crate::RefCount;
use crate::Float;
use crate::material::Material;
use crate::sampleable_trait::Sampleable;
use crate::interaction::{Interaction,SurfaceInteractionData, ShadingInfo};
use geometry3d::intersect_trait::{IntersectionInfo};
use geometry3d::{Ray3D,  Transform };
use crate::ray::Ray;

type Texture = fn(Float,Float)->Float;

pub struct Object {
    pub primitive: Box<dyn Sampleable>,
    pub front_material_index: usize,
    pub back_material_index: usize,
    pub texture: Option<RefCount<Transform>>,
}


#[derive(Default)]
pub struct Scene {
    /// Objects in the scene that are not tested
    /// directly for shadow (e.g., non-luminous objects
    /// and diffuse light)
    pub objects: Vec<RefCount<Object>>,
    
    /// The materials in the scene
    pub materials: Vec<RefCount<dyn Material>>,

    /// A vector of [`Light`] objects that
    /// are considered sources of direct light
    pub lights: Vec<RefCount<Object>>,

    /// A vector of [`Light`] objects that
    /// are considered sources of direct light
    pub distant_lights: Vec<RefCount<Object>>,

    pub transforms: Vec<RefCount<Transform>>,

    pub textures: Vec<RefCount<Texture>>,
}

impl Scene {
    /// Creates an empty scene
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of total lights; that is, 
    /// those in the `lighs` field and those in the `distant_lights` 
    /// one
    pub fn count_all_lights(&self)->usize{
        self.lights.len() + self.distant_lights.len()
    }

   
    /// Casts a [`Ray3D`] and returns an `Option<(Float,Vector3D,usize)>` in which the
    /// [`Vector3D`] is the normal at the point of intersection, the `usize`
    /// is the index of the [`Material`] encountered, and the `Float` is the distance 
    /// that the [`Ray3D`] travelled to it.    
    pub fn cast_ray(&self, ray: &Ray) -> Option<(Float,Interaction)> {
        const MIN_T: Float = 0.000001;

        let mut t_squared = Float::MAX;
        // let mut material_index = usize::MAX;
        let mut intersected = false;
        let mut info : Option<IntersectionInfo> = None;
        let mut object: Option<RefCount<Object>> = None;
        
        // let mut is_object = false;
        // let mut is_light = false;
        // let mut is_distant_light = false;

        // Test objects
        for this_object in self.objects.iter() {
            if let Some(intersection_info) = this_object.primitive.intersect(&ray.geometry) {
                
                let this_t_squared = (intersection_info.p - ray.origin()).length_squared();

                // Is it a valid hit and it is earlier than the rest?
                if this_t_squared > MIN_T && this_t_squared < t_squared {                    
                    t_squared = this_t_squared;
                    object = Some(RefCount::clone(this_object));                    
                    info = Some(intersection_info);
                    intersected = true;
                    // is_object = true;
                }
            }
        }

        // Test lights
        for this_object in self.lights.iter() {
            if let Some(intersection_info) = this_object.primitive.intersect(&ray.geometry) {
                let this_t_squared = (intersection_info.p - ray.origin()).length_squared();
                // Is it a valid hit and it is earlier than the rest?
                if this_t_squared > MIN_T && this_t_squared < t_squared {
                    // Update info.
                    
                    t_squared = this_t_squared;
                    info = Some(intersection_info);
                    object = Some(RefCount::clone(this_object));                    
                    intersected = true;
                    // is_light = true;
                }
            }
        }

        // if no intersection yet
        if !intersected{
            for this_object in self.distant_lights.iter() {
                if let Some(intersection_info) = this_object.primitive.intersect(&ray.geometry) {
                    let this_t_squared = (intersection_info.p - ray.origin()).length_squared();
                    // Is it a valid hit and it is earlier than the rest?
                    if this_t_squared > MIN_T && this_t_squared < t_squared {
                        // Update info.
                        
                        t_squared = this_t_squared;
                        info = Some(intersection_info);
                        object = Some(RefCount::clone(this_object));                        
                        intersected = true;
                        // is_distant_light = true;
                    }
                }
                
            }
        }

        // Return
        if !intersected {
            None
        } else {
            let info = info.unwrap();
            let object = object.unwrap();
            let t = t_squared.sqrt();  
                    
            let point = ray.geometry.project(t);
            let data = SurfaceInteractionData{
                point,
                // perror: info.perror,
                time: ray.time,
                wo: ray.geometry.direction * -1.,
                geometry_shading: ShadingInfo{
                    u: info.u,
                    v: info.v,
                    normal: info.normal,
                    dpdu: info.dpdu,
                    dpdv: info.dpdv,
                    dndu: info.dndu,
                    dndv: info.dndv,
                    side: info.side
                },
                texture_shading: None,
                object,
            };

            Some((t,Interaction::Surface(data)))
        }
    }

    /// Checks whether a [`Ray3D`] can travel a certain distance without being obstructed
    pub fn unobstructed_distance(&self, ray: &Ray3D, distance: Float) -> bool {
        const MIN_T: Float = 1e-20;
        let d_squared = distance * distance;

        debug_assert!((1. - ray.direction.length()).abs() < 0.00000001);

        // Check all objects
        for object in self.objects.iter() {
            // If it intersects an object,
            if let Some(pt) = object.primitive.simple_intersect(&ray) {                
                let this_d_squared = (pt - ray.origin).length_squared();

                // Is it a valid hit and it is earlier than the rest?
                if this_d_squared > MIN_T && this_d_squared + MIN_T < d_squared && (d_squared - this_d_squared).abs() > 0.0001 {
                    return false;
                }
            }
        }

        // Check lights as well
        for object in self.lights.iter() {
            // If it intersects an object,
            if let Some(pt) = object.primitive.simple_intersect(&ray) {
                let this_d_squared = (pt - ray.origin).length_squared();
                // Is it a valid hit and it is earlier than the rest?
                if this_d_squared > MIN_T && this_d_squared + MIN_T < d_squared && (d_squared - this_d_squared).abs() > 0.0001 {
                    return false;
                }
            }
        }

        // it is unobstructed
        true
    }

    /// Pushes a [`Material`] to the [`Scene`]
    pub fn push_material(&mut self, material: RefCount<dyn Material>) -> usize {
        self.materials.push(material);
        // return
        self.materials.len() - 1
    }


    
    /// Pushes an [`Object`] into the [`Scene`]
    pub fn push_object(
        &mut self,
        front_material_index: usize,
        back_material_index: usize,
        object: Box<dyn Sampleable>,
    ) -> usize {
        if front_material_index >= self.materials.len() {
            panic!("Pushing object with front material out of bounds")
        }

        if back_material_index >= self.materials.len() {
            panic!("Pushing object with back material out of bounds")
        }

        let this_index = self.objects.len();
        let ob_id = object.id();
        let object = Object {
            front_material_index,
            back_material_index,
            primitive: object,
            
            texture: None,            
        };

        
        // Mark as source
        if self.materials[front_material_index].emits_direct_light()
            || self.materials[back_material_index].emits_direct_light()
        {
            
            // I know this is not very fast... but we will
            // only do this while creating the scene, not while
            // rendering
            if ob_id == "source"{                 
                self.distant_lights.push(RefCount::new(object));
            }else{
                self.lights.push(RefCount::new(object))
            }        
        }else{
            // Push
            self.objects.push(RefCount::new(object));
        }

        // return
        this_index
    }

    

    
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_shadow_ray() {
    //     // assert!(false)
    // }
}

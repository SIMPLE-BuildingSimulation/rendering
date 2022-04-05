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
use crate::bvh::BoundingVolumeTree;
use crate::from_simple_model::SimpleModelReader;
use crate::material::Material;
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::Float;
use geometry3d::Ray3D;
// type Texture = fn(Float,Float)->Float;
use simple_model::SimpleModel;

#[derive(Clone)]
pub struct Object {
    pub primitive: Primitive,
    pub front_material_index: usize,
    pub back_material_index: usize,
    // pub texture: Option<RefCount<Transform>>,
}

#[derive(Default)]
pub struct Scene {
    /// Objects in the scene that are not tested
    /// directly for shadow (e.g., non-luminous objects
    /// and diffuse light)
    pub objects: Vec<Object>,

    /// The materials in the scene
    pub materials: Vec<Box<dyn Material + Sync>>,

    /// A vector of [`Light`] objects that
    /// are considered sources of direct light.
    /// The objects here are also in the objects part.
    pub lights: Vec<Object>,

    /// A vector of [`Light`] objects that
    /// are considered sources of direct light
    pub distant_lights: Vec<Object>,

    /// The acceleration structure that helps trace rays.
    /// 
    /// This needs to be build through the `build_accelerator` function.
    pub accelerator : Option<BoundingVolumeTree>,

    
}

pub enum Wavelengths {
    Solar,
    Visible,
}

impl Scene {
    /// Creates a new `Scene` from a `SimpleModel`. The `enum` `Wavelengths`
    /// can be used to create a `Visible` or a `Solar` model, for calculating
    /// Lighting or Solar Radiation, respectively.
    pub fn from_simple_model(model: &SimpleModel, wavelength: Wavelengths) -> Self {
        let mut reader = SimpleModelReader::default();
        reader.build_scene(model, &wavelength)
    }

    /// Creates an empty scene
    pub fn new() -> Self {
        Self::default()
    }

    pub fn build_accelerator(&mut self) {
        if self.accelerator.is_some(){
            panic!("Trying to re-build accelerator structure. If you really want this, use rebuild_accelerator")
        }
        self.accelerator = Some(BoundingVolumeTree::new(self));
    }

    /// Builds the accelerator
    pub fn rebuild_accelerator(&mut self) {        
        self.accelerator = Some(BoundingVolumeTree::new(self));
    }

    /// Returns the number of total lights; that is,
    /// those in the `lighs` field and those in the `distant_lights`
    /// one
    pub fn count_all_lights(&self) -> usize {
        self.lights.len() + self.distant_lights.len()
    }

    /// Casts a [`Ray3D`] and returns an `Option<Interaction>` describing the
    /// interaction with the first primitive hit by the ray, if any.    
    pub fn cast_ray(&self, ray: &mut Ray, node_aux: &mut Vec<usize>) -> bool {        
        if let Some(accelerator) = &self.accelerator{
            accelerator.intersect(&self.objects, ray, node_aux)
        }else{
            panic!("")
        }
        
    }

    /// Checks whether a [`Ray3D`] can travel a certain distance without hitting any surface
    pub fn unobstructed_distance(&self, ray: &Ray3D, distance_squared: Float,  node_aux: &mut Vec<usize>) -> bool {
        
        if let Some(a)=&self.accelerator{
            a.unobstructed_distance(&self.objects, ray, distance_squared, node_aux)            
        }else{
            panic!("Trying to cast a check if unobstructed_distance() in a scene without an acceleration structure")
        }
    }

    /// Pushes a [`Material`] to the [`Scene`] and return its
    /// position in the `materials` Vector.
    pub fn push_material(&mut self, material: Box<dyn Material + Sync>) -> usize {
        self.materials.push(material);
        // return
        self.materials.len() - 1
    }

    /// Pushes an [`Object`] into the [`Scene`]
    pub fn push_object(
        &mut self,
        front_material_index: usize,
        back_material_index: usize,
        primitive: Primitive,
    ) -> usize {
        if front_material_index >= self.materials.len() {
            panic!("Pushing object with front material out of bounds")
        }

        if back_material_index >= self.materials.len() {
            panic!("Pushing object with back material out of bounds")
        }

        let this_index = self.objects.len();
        let ob_id = primitive.id();
        let object = Object {
            front_material_index,
            back_material_index,
            primitive,
            // texture: None,
        };

        // Mark as source
        if self.materials[front_material_index].emits_direct_light()
            || self.materials[back_material_index].emits_direct_light()
        {
            // I know this is not very fast... but we will
            // only do this while creating the scene, not while
            // rendering
            if ob_id == "source" {
                self.distant_lights.push(object);
            } else {
                // register object as light
                self.lights.push(object.clone());
                // Push object
                self.objects.push(object)
            }
        } else {
            // Push
            self.objects.push(object);
        }

        // return
        this_index
    }
}

#[cfg(test)]
mod tests {

    // #[test]
    // fn test_push_material() {
    //     // Add a material

    //     // Add the material again

    //     // The number of materials should be 1.

    //     // Both indexes should be the same (1)

    //     assert!(false)
    // }
}

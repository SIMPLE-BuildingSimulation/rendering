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
use crate::colour::Spectrum;
use crate::scene::Scene;
use geometry3d::point3d::Point3D;
use geometry3d::vector3d::Vector3D;
use geometry3d::ray3d::Ray3D;

/// # Note
/// THESE ROUTINES ARE GREATLY INFLUENCED BY ERIC VEACH'S PHD THESIS AND BY
/// THE [PBR-BOOK](https://pbr-book.org/)

/// The data within a Vertex.
#[derive(Clone, Copy)]
pub struct VertexData {
    /// The normal at the interaction point
    pub normal: Option<Vector3D>,

    /// The position of the [`Vertex`] in the [`Scene`]
    pub position: Point3D,

    /// The index of the [`Material`] associated with the interaction
    /// within the `materials` vector in the [`Scene`]
    pub material_index: Option<usize>,

    /// The index of the [`Object`] associated with the interaction
    /// within the `materials` vector in the [`Scene`]
    pub object_index: Option<usize>,

    /// A flag indicating whether the [`Vertex`] (from page 315 of Veach's PhD Thesis)
    pub is_specular: bool,

    // beta:Spectrum//, alpha?
    // forward_pdf:Float
    // backward_pdf:Float
}

/// The kind of [`Vertex`], useful for
/// distinguishing the different ways of calculating
/// different probabilities and sampling.
#[derive(Clone, Copy)]
pub enum Vertex {
    /// A [`Vertex`] in the camera lens
    Camera(VertexData),

    /// A [`Vertex`] in the Light object
    Light(VertexData),

    /// A [`Vertex`] on a surface
    Surface(VertexData),
    // Not implemented yet
    //Medium(VertexData),
}

impl Vertex {
    pub fn open_data(&self) -> &VertexData {
        match self {
            Self::Camera(data) => data,
            Self::Light(data) => data,
            Self::Surface(data) => data,
        }
    }

    pub fn new_in_camera(ray:Ray3D,view_direction: Vector3D,beta:Spectrum)->Self{
        Self::Camera(VertexData{
            normal: Some(view_direction),
            position: ray.origin,
            material_index: None,
            object_index: None,
            is_specular: false,
        })
    }

    pub fn new_in_light(ray:Ray3D,position: Point3D, beta:Spectrum)->Self{
        Self::Camera(VertexData{
            normal: Some(view_direction),
            position: ray.origin,
            material_index: None,
            object_index: None,
            is_specular: false,
        })
    }

    /// get position
    pub fn position(&self) -> Point3D {
        self.open_data().position
    }
    /// get normal
    pub fn normal(&self) -> Option<Vector3D> {
        self.open_data().normal
    }

    /// Checks whether this [`Vertex`] is is associated with a
    /// Light material and has infinite size
    ///
    /// Adapted from the [pbr-book](https://pbr-book.org/)
    pub fn is_infinite_light(&self, scene: &Scene) -> bool {
        match self {
            Self::Light(data) => {
                scene
                    .borrow_material(data.material_index.unwrap()) // It should have a material
                    .emits_direct_light()
                    && scene
                        .borrow_object(data.object_index.unwrap())
                        .is_infinite() // it should have an associated object
            }
            _ => false,
        }
    }

    /// Checks whether the [`Vertex`] emits light
    ///
    /// Adapted from the [pbr-book](https://pbr-book.org/)
    pub fn is_light(&self, scene: &Scene) -> bool {
        match self {
            Self::Light(_) => true,
            Self::Surface(data) => scene
                .borrow_material(data.material_index.unwrap())
                .emits_light(),
            _ => false,
        }
    }

    pub fn is_delta_light(&self) -> bool {
        // Don't allow delta lights for now
        false
    }

    /// Checks whether it makes sense to connect to this [`Vertex`]
    ///
    /// Adapted from the [pbr-book](https://pbr-book.org/)
    pub fn is_connectible(&self, scene: &Scene) -> bool {
        match self {
            //Self::Medium(_)=>true,
            Self::Camera(_) => true,
            Self::Surface(data) => scene
                .borrow_material(data.material_index.unwrap())
                .specular_only(),
            Self::Light(_) => {
                // Delta lights are not connectible
                !self.is_delta_light()
            }
        }
    }

    pub fn le(&self, scene: &Scene, other: &Vertex) -> Spectrum {
        if !self.is_light(scene) {
            return Spectrum::black();
        }

        // let w = (other.position() - self.position()).get_normalized();
        // if self.is_infinite_light(scene){

        // }else{

        // it is light, it should have a material
        let mat_index = self.open_data().material_index.unwrap();
        let m = scene.borrow_material(mat_index);
        debug_assert!(m.emits_light());
        // Only uniform lights are allowed for now
        m.colour()
        // }
    }
}

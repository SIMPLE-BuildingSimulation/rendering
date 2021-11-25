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
use crate::scene::Scene;
use crate::vertex::Vertex;

#[derive(Eq, PartialEq)]
#[repr(u8)]
pub enum TransportMode{
    Importance,
    Radiance,
}

const MAX_VERTICES: usize = 12;

#[derive(Clone, Copy)]
pub struct SubPath {
    vertices: [Option<Vertex>; MAX_VERTICES],
    pub n_vertices: usize,
}

impl std::ops::Index<usize> for SubPath {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= MAX_VERTICES {
            panic!(
                "Trying to acces Vertex out of bounds in SubPath: index is {} and len() is {}",
                index,
                self.vertices.len()
            );
        }
        match &self.vertices[index] {
            Some(v) => v,
            None => panic!(
                "Trying to access a None vertex in Subpath... index is {}",
                index
            ),
        }
    }
}

impl SubPath {
    /// Creates a new empty path
    pub fn new() -> Self {
        Self {
            vertices: [None; MAX_VERTICES],
            n_vertices: 0,
        }
    }

    pub fn random_walk(&mut self, scene: &Scene, max_depth: usize, rroulet: Float, pdf: Float, beta: Spectrum, transport_mode: TransportMode, ray: Ray) {
        if max_depth == 0{
            return
        }
        let mut bounces : u8 = 0;
        let pdf_fwd = pdf;
        let mut any_non_specular_bounce = false;
        
        loop {
            // if !beta {break;}// I am not sure what this means.

            let mut scattered = false;
            let mut terminated = false;
            let vertex = &self[bounces];
            let prev = &self[bounces - 1];

            if let Some((thit, interaction)) = scene.cast_ray(&ray){
                // If we hit, 
                // if (ray.medium) {
                    /* SKIPPED... no mediums yet */
                // }
                if terminated { return }
                if scattered { continue }

                match interaction {
                    Interaction::Surface(data)=>{                        
                        // get the normal... can be textured.           
                        let normal = data.normal();

                        debug_assert!((1.0 - normal.length()).abs() < 0.000001);

                        // let object = interaction.object();

                        let material_index = match data.geometry_shading.side {
                            SurfaceSide::Front => {
                                object.front_material_index
                            },
                            SurfaceSide::Back =>{
                                object.back_material_index
                            }                        
                        };
                        let material = &scene.materials[material_index];

                        let intersection_pt = ray.geometry.project(t);
                        let vertex_data = VertexData{
                            normal: Some(normal),
                            position: data.point,
                            material_index: Some(material_index),
                            object_index: Some(object.index),
                            is_specular:material.specular_only,
                            beta,
                            forward_pdf:pdf_fwd,
                            backward_pdf:0.,//?
                        };
                        self[bounces + 1] = Vertex::new_in_surface(vertex_data);

                        

                        let ray_dir = ray.geometry.direction;
                        let new_ray_dir = material.sample_bsdf(ray_dir, normal);
                        debug_assert!((1.-new_ray_dir.length()).abs() < 0.0000001);
                        let new_ray = Ray{
                            time: ray.time,
                            geometry: Ray3D {
                                direction : new_ray_dir,
                                origin: intersection_pt,// + normal * 0.0001, // avoid self shading
                            }
                        };
                        
                    },
                    // Interaction::Medium()=>{}
                    Interaction::Endpoint(data)=>{panic!("Encountered an Endpoint Interaction when doing a random walk")}
                }
                



            }else{
                // Capture escaped rays when tracing from the camera
                // if (mode == TransportMode::Radiance) {
                //     vertex = Vertex::CreateLight(EndpointInteraction(ray), beta, pdfFwd);
                //     ++bounces;
                // }
                
                break;
            }

        }
    }

    /// Pushes a new [`Vertex`]; returns the index of that [`Vertex`]
    pub fn push(&mut self, v: Vertex) -> usize {
        // This needs to be None, for now.
        debug_assert!(self.vertices[self.n_vertices].is_none());
        // Now we initialize it
        self.vertices[self.n_vertices] = Some(v);
        self.n_vertices += 1;
        self.n_vertices - 1
    }

    // Extends both sides of the path
    // fn alternate_walk(light_subpath:&mut Self, max_source_subpath_steps: usize, eye_subpath:&mut Self, max_eye_subpath_steps: usize, rroulete: Float){
    //     let this_source_ver = light_subpath.last();
    //     let this_eye_vert

    //     // mark the first two points from eye

    //     // Random walk

    // }

    // Evaluates the path combinations
    // fn evaluate(light_subpath:&Self, eye_subpath:&Self)->Spectrum{

    // }
}

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
use crate::colour::Spectrum;
use crate::image::ImageBuffer;
use crate::scene::Scene;
use crate::subpath::SubPath;
use crate::vertex::{Vertex, VertexData};




pub struct BidPathTracer {
    max_depth: usize
}

impl BidPathTracer {
    /// Process a single camera ray
    pub fn render(scene: &Scene, camera: &dyn Camera) -> ImageBuffer {        
        // const RROULETE: Float = 0.1;
        

        let (width, height) = camera.film_resolution();
        let mut buffer = ImageBuffer::new(width, height);
        let total_pixels = width * height;

        let mut last_progress: Float = 0.0;
        let mut i = 0;

        if scene.n_lights() == 0 {
            return samples;
        }

        for y in 0..height {
            for x in 0..width {
                let mut l = Spectrum::black();
                
                /*
                Veach's thesis, p. 298

                Each technique samples a path by connecting two independently
                generated pieces, one starting from the light sources, and the
                other from the eye.
                */
                let light_subpath =
                    get_light_subpath(light_i, scene, MAX_SOURCE_SUBPATH_DEPTH, RROULETE);
                let eye_subpath =
                    get_eye_subpath(scene, camera, MAX_EYE_SUBPATH_DEPTH, RROULETE);
                evaluate_path(&mut samples, &light_subpath, &eye_subpath);

                // from PBR book
                execute_connections(&eye_subpath, &light_subpath)
            } // end of x
        } // end of y

        buffer
    }// end of render

    fn execute_connections(&self, eye_subpath:&SubPath, light_subpath:&SubPath, buffer: &mut ImageBuffer){
        for t in 1..=eye_subpath.len() {
            for s in 0..=light_subpath.len() {
                let depth = t + s - 2;
                if (s == 1 && t == 1) || depth < 0 || depth > self.max_depth {
                    continue;
                }
    
                // Connect and update                        
                let (l_path, weight) = connect_paths(
                    scene, 
                    light_subpath, 
                    eye_subpath, 
                    s, t, 
                    // light_distribution,
                    // camera,
                    // tile_sampler,                             
                );
    
                if t != 1 {
                    l += l_path
                }else{
                    buffer[(x, y)] = splat(, l_path)
                }
            }   
        }// end of paths
        buffer[(x, y)] = l;
    }

    fn random_walk(&self){

    }
    
    fn evaluate_path(&self,
        samples: &mut ImageBuffer,
        light_subpath: &SubPath,
        eye_subpath: &SubPath,
    ) -> Spectrum {
        unimplemented!();
    }
    
    /// Generates the Light [`Subpath`]
    ///
    /// From Veach's thesis, p. 298: "the light subpath... is constructed by choosing a random point... on
    /// a light source, followed by casting a ray in a random direction"
    fn get_light_subpath(&self, light_i: usize, scene: &Scene, max_depth: usize, rroulet: Float) -> SubPath {
        let object_index = scene.light(light_i);
        let light = scene.borrow_object(object_index);
        // There must be a more efficient way of doing this..?
        let light_sampler = light.primitive.surface_sampler(1);
        let light_p = light_sampler.next().unwrap();
    
        // We are assuming that the light emits uniformly
        let mut ret = SubPath::new();    
        ret.push(Vertex::Camera(VertexData {
            normal: None,
            position: light_p,
            material_index: Some(light.front_material_index),
            object_index: None,
            is_specular: false,
        }));
    
    }
    
    /// Generates the Eye [`Subpath`]
    ///
    /// From Veach's thesis, p. 298: "The eye subpath ... is constructed by a similar process starting
    /// from a random point on the camera lens."
    fn get_eye_subpath(&self, scene: &Scene, camera: &dyn Camera, max_depth: usize, rroulet: Float) -> SubPath {
        
        // Get a camera vertex
        let (x_pos, y_pos) = rand::random::<(Float, Float)>();
        let (f_width, f_height) = camera.film_resolution();
        let x = (x_pos * f_width as Float).round() as usize;
        let y = (y_pos * f_height as Float).round() as usize;
        debug_assert!(x <= f_width);
        debug_assert!(y <= f_height);
        let sample = CameraSample {
            p_film: (x, y),
            p_lens: (0., 0.), // we still do not use this
            time: 0.,         // we still do not use this
        };
        let (ray, _weight) = camera.gen_ray(&sample);
    
        let mut ret = SubPath::new();
        let view = camera.view();
        let beta = Spectrum::black();// this is probably wrong
        ret.push(Vertex::new_in_camera(ray,view.view_direction, beta));
    
        // Random walk
        ret.random_walk(scene, max_depth - 1, rroulet);
        // return
        ret
    }
    

}

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

use geometry3d::Triangulation3D;
use simple_model::{SimpleModel, Substance};

use crate::colour::Spectrum;
use crate::material::{Material, Plastic};
use crate::primitive::Primitive;
use crate::scene::{Scene, Wavelengths};

/// An auxiliar structure only meant to create a Scene from a SimpleModel
#[derive(Default)]
pub struct SimpleModelReader {
    /// A list of the modifiers already in the model
    modifiers: Vec<String>,
}

impl SimpleModelReader {
    pub fn build_scene(&mut self, model: &SimpleModel, wavelength: &Wavelengths) -> Scene {
        if matches!(wavelength, Wavelengths::Visible) {
            unimplemented!()
        }

        let mut scene = Scene::new();

        // Add surfaces
        for s in &model.surfaces {
            let polygon = &s.vertices;
            let construction = &s.construction;
            // Should not be empty, and should have been check before this
            assert!(!construction.materials.is_empty());

            let front_substance = &construction.materials[0].substance;
            let front_mat_index = self.push_substance(&mut scene, front_substance, wavelength);
            let back_substance = &construction.materials.last().unwrap().substance; // again, this would have been checked.
            let back_mat_index = self.push_substance(&mut scene, back_substance, wavelength);

            // Add all the triangles necessary
            let triangles = Triangulation3D::from_polygon(polygon)
                .unwrap()
                .get_trilist();
            for tri in triangles {
                scene.push_object(front_mat_index, back_mat_index, Primitive::Triangle(tri));
            }
        }

        // Add fenestrations
        for s in &model.fenestrations {
            let polygon = &s.vertices;
            let construction = &s.construction;
            // Should not be empty, and should have been check before this
            assert!(!construction.materials.is_empty());

            let front_substance = &construction.materials[0].substance;
            let front_mat_index = self.push_substance(&mut scene, front_substance, wavelength);
            let back_substance = &construction.materials.last().unwrap().substance; // again, this would have been checked.
            let back_mat_index = self.push_substance(&mut scene, back_substance, wavelength);

            // Add all the triangles necessary
            let triangles = Triangulation3D::from_polygon(polygon)
                .unwrap()
                .get_trilist();
            for tri in triangles {
                scene.push_object(front_mat_index, back_mat_index, Primitive::Triangle(tri));
            }
        }

        // return
        scene
    }

    /// Adds a Substance to the Scene, checking if it has been added before (by name).
    /// If a substance has already been added to the Scene, then it will not add it.
    ///
    /// Returns the index of the already existing or new Material in the Scene.
    fn push_substance(
        &mut self,
        scene: &mut Scene,
        substance: &Substance,
        wavelength: &Wavelengths,
    ) -> usize {
        let substance_name = substance.name().to_string();
        match self.get_modifier_index(&substance_name) {
            Some(i) => i,
            None => {
                // Material is not there... add, then.
                let front_mat = Self::substance_to_material(substance, wavelength);
                scene.push_material(front_mat)
            }
        }
    }

    fn get_modifier_index(&self, item: &str) -> Option<usize> {
        for (i, v) in self.modifiers.iter().enumerate() {
            if v == item {
                return Some(i);
            }
        }
        None // not found
    }

    /// Transformsa a SimpleModel Substance into a Material
    fn substance_to_material(substance: &Substance, wavelength: &Wavelengths) -> Box<dyn Material + Sync> {
        if matches!(wavelength, Wavelengths::Visible) {
            unimplemented!();
        }

        let color = match substance {
            Substance::Normal(s) => {
                let alpha = match s.solar_absorbtance() {
                    Ok(v) => *v,
                    Err(_) => {
                        let v = 0.7;
                        eprintln!("Substance '{}' does not have a Solar Absorbtance... assuming value of {}", s.name, v);
                        v
                    }
                };
                // return solar reflection
                1. - alpha
            }
        };

        // return
        Box::new(Plastic {
            colour: Spectrum::gray(color),
            specularity: 0.0,
            roughness: 0.0,
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::material::{ Light};
    use super::*;
    use crate::camera::{Film, Pinhole, View};
    use crate::ray_tracer::RayTracer;
    use crate::Float;
    use geometry3d::{DistantSource3D, Point3D, Vector3D};
    use std::time::Instant;

    #[test]
    #[ignore]
    fn test_scene_from_model() {
        // BUILD SCENE
        let (model, _state_header) =
            SimpleModel::from_file("./test_data/room.spl".to_string()).unwrap();
        let mut reader = SimpleModelReader::default();
        let mut scene = reader.build_scene(&model, &Wavelengths::Solar);

        let light_index = scene.push_material(Box::new(Light(Spectrum::gray(10000.))));
        scene.push_object(
            light_index,
            light_index,
            Primitive::Source(DistantSource3D::new(
                Vector3D::new(1., 1., 1.),   // direction
                (0.5 as Float).to_radians(), // angle
            )),
        );

        // RENDER
        scene.build_accelerator();

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
        let camera = Pinhole::new(view, film);

        let integrator = RayTracer {
            n_ambient_samples: 220,
            n_shadow_samples: 1,
            max_depth: 3,
            ..RayTracer::default()
        };

        let now = Instant::now();

        let buffer = integrator.render(&scene, &camera);
        println!("Room took {} seconds to render", now.elapsed().as_secs());
        buffer.save_hdre(std::path::Path::new("./test_data/images/simple_room.hdr"));
    }
}

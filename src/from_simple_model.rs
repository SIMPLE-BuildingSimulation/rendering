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
use crate::material::{Glass, Material, Plastic};
use crate::primitive::Primitive;
use crate::scene::{Scene, Wavelengths};

/// An auxiliar structure only meant to create a Scene from a SimpleModel
#[derive(Default)]
pub struct SimpleModelReader {
    /// A list of the modifiers already in the model
    modifiers: Vec<String>,
}

fn transmittance_to_transmissivity(tau: crate::Float) -> crate::Float {
    ((0.8402528435 + 0.0072522239 * tau.powi(2)).sqrt() - 0.9166530661) / 0.0036261119 / tau
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
            assert!(
                !construction.materials.is_empty(),
                "Found an empty construction, called {}",
                construction.name()
            );

            let front_substance = &construction.materials[0].substance;
            let front_mat_index = self
                .push_substance(&mut scene, front_substance, wavelength)
                .unwrap_or_else(|| {
                    panic!(
                    "Front material of  Construction '{}' seems to be a gas. This is not supported",
                    construction.name()
                )
                });
            let back_substance = &construction.materials.last().unwrap().substance; // again, this would have been checked.
            let back_mat_index = self
                .push_substance(&mut scene, back_substance, wavelength)
                .unwrap_or_else(|| {
                    panic!(
                    "Back material of  Construction '{}' seems to be a gas. This is not supported",
                    construction.name()
                )
                });

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
            assert!(
                !construction.materials.is_empty(),
                "Found an empty construction, called {}",
                construction.name()
            );

            let front_substance = &construction.materials[0].substance;
            let front_mat_index = self
                .push_substance(&mut scene, front_substance, wavelength)
                .unwrap_or_else(|| {
                    panic!(
                    "Front material of  Construction '{}' seems to be a gas. This is not supported",
                    construction.name()
                )
                });
            let back_substance = &construction.materials.last().unwrap().substance; // again, this would have been checked.
            let back_mat_index = self
                .push_substance(&mut scene, back_substance, wavelength)
                .unwrap_or_else(|| {
                    panic!(
                    "Back material of  Construction '{}' seems to be a gas. This is not supported",
                    construction.name()
                )
                });

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
    ) -> Option<usize> {
        let substance_name = substance.name().to_string();
        match self.get_modifier_index(&substance_name) {
            Some(i) => Some(i),
            None => {
                // Material is not there... add, then.
                let front_mat = Self::substance_to_material(substance, wavelength)?;
                Some(scene.push_material(front_mat))
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
    fn substance_to_material(substance: &Substance, wavelength: &Wavelengths) -> Option<Material> {
        match substance {
            Substance::Normal(s) => {
                let alpha = match *wavelength {
                    Wavelengths::Solar => match s.front_solar_absorbtance() {
                        Ok(v) => *v,
                        Err(_) => {
                            let v = 0.7;
                            eprintln!("Substance '{}' does not have a Solar Absorbtance... assuming value of {}", s.name, v);
                            v
                        }
                    },
                    Wavelengths::Visible => match s.front_visible_reflectance() {
                        Ok(v) => *v,
                        Err(_) => {
                            let v = 0.7;
                            eprintln!("Substance '{}' does not have a Solar Absorbtance... assuming value of {}", s.name, v);
                            v
                        }
                    },
                };
                let rho = 1. - alpha;
                let tau = match *wavelength {
                    Wavelengths::Solar => match s.solar_transmittance() {
                        Ok(v) => transmittance_to_transmissivity(*v),
                        Err(_) => {
                            let v = 0.;
                            eprintln!("Substance '{}' does not have a Solar Absorbtance... assuming value of {}", s.name, v);
                            v
                        }
                    },
                    Wavelengths::Visible => match s.visible_transmissivity() {
                        Ok(v) => *v,
                        Err(_) => {
                            let v = 0.;
                            eprintln!("Substance '{}' does not have a Solar Absorbtance... assuming value of {}", s.name, v);
                            v
                        }
                    },
                };

                // return
                if tau > 0.0 {
                    Some(Material::Glass(Glass {
                        colour: Spectrum::<{ crate::N_CHANNELS }>::gray(tau),
                        refraction_index: 1.52,
                    }))
                } else {
                    Some(Material::Plastic(Plastic {
                        colour: Spectrum::<{ crate::N_CHANNELS }>::gray(rho),
                        specularity: 0.0,
                        roughness: 0.0,
                    }))
                }
            }
            Substance::Gas(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::camera::{Film, Pinhole, View};
    use crate::material::Light;
    use crate::ray_tracer::RayTracer;
    use crate::Float;
    use geometry3d::{DistantSource3D, Point3D, Vector3D};
    use std::time::Instant;
    use validate::assert_close;

    #[test]
    fn test_transmittance_to_transmissivity() {
        assert_close!(0.96, transmittance_to_transmissivity(0.88), 1e-2)
    }

    #[test]
    #[ignore]
    fn test_scene_from_model() {
        // BUILD SCENE
        let (model, _state_header) =
            SimpleModel::from_file("./tests/scenes/room.spl".to_string()).unwrap();
        let mut reader = SimpleModelReader::default();
        let mut scene = reader.build_scene(&model, &Wavelengths::Solar);

        let light_index = scene.push_material(Material::Light(Light(Spectrum::<
            { crate::N_CHANNELS },
        >::gray(10000.))));
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
        buffer.save_hdre(std::path::Path::new(
            "./tests/scenes/images/simple_room.hdr",
        ));
    }
}

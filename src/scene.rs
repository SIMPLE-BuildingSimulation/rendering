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
use crate::colour::Spectrum;
use crate::from_simple_model::SimpleModelReader;
use crate::material::{Light, Material};
use crate::primitive::Primitive;
use crate::ray::Ray;
use crate::triangle::Triangle;
use crate::Float;
use calendar::Date;
use geometry3d::{Ray3D, Vector3D};
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
    /// The Triangles in the scene that are not tested
    /// directly for shadow (e.g., non-luminous objects
    /// and diffuse light)    
    pub triangles: Vec<Triangle>,

    /// The normal of each vertex of each triangle.
    pub normals: Vec<(Vector3D, Vector3D, Vector3D)>,

    pub front_material_indexes: Vec<usize>,

    pub back_material_indexes: Vec<usize>,

    /// The materials in the scene
    pub materials: Vec<Material>,

    /// A vector of [`Light`] objects that
    /// are considered sources of direct light.
    /// The objects here are also in the objects part.
    pub lights: Vec<Object>,

    /// A vector of distant [`Light`] objects that
    /// are considered sources of direct light
    pub distant_lights: Vec<Object>,

    /// The acceleration structure that helps trace rays.
    ///
    /// This needs to be build through the `build_accelerator` function.
    pub accelerator: Option<BoundingVolumeTree>,

    /// The colour of the sky, normalized
    pub sky_colour: Option<Spectrum<{ crate::N_CHANNELS }>>,

    /// A function returning the diffuse Sky brightness (i.e., without the sun)
    /// The sun should be added separately.
    /// Alternatively, you can use the `add_perez_sky` function
    pub sky: Option<Box<dyn Fn(Vector3D) -> Float + Sync>>,
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

    /// Adds the elements describing a Perez sky to the scene.
    /// The angles of Latitude, Longitude and Standard meridian should come
    /// in Degrees
    pub fn add_perez_sky(
        &mut self,
        date: Date,
        latitude: Float,
        longitude: Float,
        standard_meridian: Float,
        diffuse_horizontal_irrad: Float,
        direct_normal_irrad: Float,
    ) {
        let dew_point = 11.;
        // Add sky
        let solar = solar::Solar::new(
            latitude.to_radians(),
            longitude.to_radians(),
            standard_meridian.to_radians(),
        );
        let s = solar::PerezSky::get_sky_func_standard_time(
            solar::SkyUnits::Visible,
            &solar,
            date,
            dew_point,
            diffuse_horizontal_irrad,
            direct_normal_irrad,
        );

        self.sky = Some(s);

        // Add sun if there is any (it might be nighttime)
        let n = solar::Time::Standard(date.day_of_year());
        if let Some(sun_position) = solar.sun_position(n) {
            let angle = (0.533 as Float).to_radians();
            let tan_half_alpha = (angle / 2.0).tan();
            let omega = tan_half_alpha * tan_half_alpha * crate::PI;

            let cos_zenit = sun_position.z;
            let zenith = if cos_zenit <= 0. {
                // Limit zenith to 90 degrees
                crate::PI / 2.
            } else if cos_zenit >= 0.9986295347545738 {
                // Limit Zenith to 3 degrees minimum
                /*
                    The threshold above is equal to (3.*PI/180.).cos()
                    would that have been optimized by the compiler?? I guess, but
                    it did not allow me to create a constant of that value... so I
                    did this just in case
                */
                (3. * crate::PI / 180.).acos()
            } else {
                cos_zenit.acos()
            };
            let apwc = solar::PerezSky::precipitable_water_content(dew_point);
            let air_mass = solar::air_mass(zenith);
            let day = solar.unwrap_solar_time(n);
            let extraterrestrial_irradiance = solar.normal_extraterrestrial_radiation(day);
            let sky_brightness = solar::PerezSky::sky_brightness(
                diffuse_horizontal_irrad,
                air_mass,
                extraterrestrial_irradiance,
            )
            .clamp(0.01, 9e9);
            let sky_clearness = solar::PerezSky::sky_clearness(
                diffuse_horizontal_irrad,
                direct_normal_irrad,
                zenith,
            )
            .clamp(-9e9, 11.9);
            let index = solar::PerezSky::clearness_category(sky_clearness);
            let dir_illum = direct_normal_irrad
                * solar::PerezSky::direct_illuminance_ratio(apwc, zenith, sky_brightness, index);

            let sun_brightness = dir_illum / omega / crate::colour::WHITE_EFFICACY;
            let sun_mat =
                self.push_material(Material::Light(Light(
                    Spectrum::<{ crate::N_CHANNELS }>::gray(sun_brightness),
                )));

            self.push_object(
                sun_mat,
                sun_mat,
                Primitive::Source(geometry3d::DistantSource3D::new(sun_position, angle)),
            );
        } // end of "if there is a sun"
    }

    pub fn build_accelerator(&mut self) {
        if self.accelerator.is_some() {
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

    /// Casts a [`Ray3D`] and returns an `Option<usize>` indicating the index
    /// of the first primitive hit by the ray, if any. The `ray` passed will now contain
    /// the Interaction
    pub fn cast_ray(&self, ray: &mut Ray, node_aux: &mut Vec<usize>) -> Option<usize> {
        if let Some(accelerator) = &self.accelerator {
            accelerator.intersect(&self.triangles, ray, node_aux)
        } else {
            panic!("Trying to cast_ray() in a scene without an acceleration structure")
        }
    }

    /// Checks whether a [`Ray3D`] can travel a certain distance without hitting any surface
    pub fn unobstructed_distance(
        &self,
        ray: &Ray3D,
        distance_squared: Float,
        node_aux: &mut Vec<usize>,
    ) -> bool {
        if let Some(a) = &self.accelerator {
            a.unobstructed_distance(&self.triangles, ray, distance_squared, node_aux)
        } else {
            panic!("Trying to check if unobstructed_distance() in a scene without an acceleration structure")
        }
    }

    /// Pushes a [`Material`] to the [`Scene`] and return its
    /// position in the `materials` Vector.
    pub fn push_material(&mut self, material: Material) -> usize {
        self.materials.push(material);
        // return
        self.materials.len() - 1
    }

    /// Pushes a [`Primitive`] object into the [`Scene`]   
    ///
    /// If the [`Primitive`] is made of a light-emmiting [`Material`], then
    /// it will be added twice: One to the normal scene, and then another to
    /// the list of light sources.
    pub fn push_object(
        &mut self,
        front_material_index: usize,
        back_material_index: usize,
        primitive: Primitive,
    ) {
        if front_material_index >= self.materials.len() {
            panic!("Pushing object with front material out of bounds")
        }

        if back_material_index >= self.materials.len() {
            panic!("Pushing object with back material out of bounds")
        }

        // If it is light
        let is_light = if self.materials[front_material_index].emits_direct_light()
            || self.materials[back_material_index].emits_direct_light()
        {
            let ob_id = primitive.id();
            let object = Object {
                front_material_index,
                back_material_index,
                primitive: primitive.clone(),
                // texture: None,
            };
            // I know this is not very fast... but we will
            // only do this while creating the scene, not while
            // rendering
            if ob_id == "source" {
                self.distant_lights.push(object);
            } else {
                // register object as light
                self.lights.push(object);
            }
            true
        } else {
            false
        };
        let (triangles, normals) = match &primitive {
            Primitive::Triangle(tr) => crate::triangle::mesh_triangle(tr),
            Primitive::Sphere(s) => crate::triangle::mesh_sphere(s),
            _ => {
                if !is_light {
                    panic!("Unsupported Primitive '{}'", primitive.id());
                } else {
                    (vec![], vec![])
                }
            }
        };
        let additional = triangles.len();
        let front = vec![front_material_index; additional];
        let back = vec![back_material_index; additional];

        self.triangles.extend_from_slice(&triangles);
        self.normals.extend_from_slice(&normals);
        self.front_material_indexes.extend_from_slice(&front);
        self.back_material_indexes.extend_from_slice(&back);
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

use crate::material::Material;
use crate::sampleable_trait::Sampleable;
use geometry3d::intersect_trait::SurfaceSide;
use geometry3d::point3d::Point3D;
use geometry3d::ray3d::Ray3D;
use geometry3d::vector3d::Vector3D;

use crate::colour::Spectrum;

struct Object {
    pub primitive: Box<dyn Sampleable>,
    pub front_material_index: usize,
    pub back_material_index: usize,
}

#[derive(Default)]
pub struct Scene {
    /// Objects in the scene that are not tested
    /// directly for shadow (e.g., non-luminous objects
    /// and diffuse light)
    objects: Vec<Object>,

    /// The materials in the scene
    materials: Vec<Box<dyn Material>>,

    /// A vector indicating which [`Object`] in the `object` field
    /// are considered sources of direct light
    lights: Vec<usize>,
}

impl Scene {
    /// Creates an empty scene
    pub fn new() -> Self {
        Self::default()
    }

    pub fn n_materials(&self) -> usize {
        self.materials.len()
    }

    pub fn n_lights(&self) -> usize {
        self.lights.len()
    }

    /// Recursively traces a ray
    pub fn trace_ray(&self, ray: &Ray3D, depth: usize /*OPTIONS */) -> Spectrum {
        // Limit bounces
        const MAX_DEPTH: usize = 2;
        if depth > MAX_DEPTH {
            return Spectrum::black();
        }
        const N_SHADOW_SAMPLES: usize = 10;
        const N_AMBIENT_SAMPLES: usize = 10;

        // If hits an object
        if let Some((t, normal, material_index)) = self.cast_ray(ray) {
            debug_assert!((1.0 - normal.length()).abs() < 0.000001);

            let material = self.borrow_material(material_index);
            let intersection_pt = ray.project(t);

            /* SAMPLE LIGHTS */
            let local = self.get_local_illumination(
                material,
                ray.direction,
                intersection_pt,
                normal,
                N_SHADOW_SAMPLES,
                N_AMBIENT_SAMPLES,
            );

            /* SAMPLE BSDF */
            let mut global = Spectrum::black();
            if !material.emits_direct_light() {
                let n_lights = self.lights.len();
                let total_samples = N_AMBIENT_SAMPLES + n_lights * N_SHADOW_SAMPLES;
                let bsdf_c = N_AMBIENT_SAMPLES as f64 / total_samples as f64;
                for _ in 0..N_AMBIENT_SAMPLES {
                    // Choose a direction.
                    let direction = material.sample_bsdf(ray.direction, normal);
                    let new_ray = Ray3D {
                        direction,
                        origin: intersection_pt + normal * 0.0001, // avoid self shading
                    };
                    let cos_theta = (normal * direction).abs();
                    let li = self.trace_ray(&new_ray, depth + 1);
                    let material_pdf = material.bsdf(ray.direction, normal, new_ray.direction);

                    let fx = (li * cos_theta) * (material.colour() * material_pdf);

                    let denominator = material_pdf * bsdf_c;

                    // add contribution
                    global += fx / denominator;
                }
                global /= total_samples as f64;
            }

            local + global
        } else {
            // Did not hit.
            Spectrum {
                red: 0.,
                green: 0.,
                blue: 0.,
            }
        }
    }

    /// Calculates the luminance produced by the direct sources in the
    /// scene
    pub fn get_local_illumination(
        &self,
        material: &Box<dyn Material>,
        vin: Vector3D,
        point: Point3D,
        normal: Vector3D,
        n_light_samples: usize,
        n_ambient_samples: usize,
    ) -> Spectrum {
        // prevent self-shading
        let origin = point + normal * 0.0001;
        let mut ret = Spectrum::black();

        let n_lights = self.lights.len();
        let total_samples = n_ambient_samples + n_lights * n_light_samples;
        let bsdf_c = n_ambient_samples as f64 / total_samples as f64;
        let light_c = n_light_samples as f64 / total_samples as f64;

        for light_index in &self.lights {
            let primitive = &self.objects[*light_index].primitive;
            let sampler = primitive.direction_sampler(point, n_light_samples);
            for light_direction in sampler {
                // Expect direction to be normalized
                debug_assert!((1. - light_direction.length()).abs() < 0.0001);

                let shadow_ray = Ray3D {
                    origin,
                    direction: light_direction,
                };

                let light_distance = match primitive.intersect(&shadow_ray) {
                    Some(d) => d,
                    None => {
                        eprintln!("Missed light {}", light_index);
                        continue;
                    }
                };

                // If the light is visible
                if self.unobstructed_distance(&shadow_ray, light_distance) {
                    let (_normal_at_light, side) =
                        primitive.normal_at_intersection(&shadow_ray, light_distance);
                    let cos_theta = (normal * light_direction).abs();

                    let light_material = match side {
                        SurfaceSide::Front => {
                            &self.materials[self.objects[*light_index].front_material_index]
                        }
                        SurfaceSide::Back => {
                            &self.materials[self.objects[*light_index].back_material_index]
                        }
                    };
                    let light_colour = light_material.colour();

                    // Denominator of the Balance Heuristic... I am assuming that
                    // when one light has a pdf>0, then all the rest are Zero... is this
                    // correct?
                    let light_pdf = 1. / primitive.omega(origin);
                    let material_pdf = material.bsdf(vin, normal, shadow_ray.direction * -1.);
                    let denominator = material_pdf * bsdf_c + light_pdf * light_c;
                    let fx = (light_colour * cos_theta) * (material.colour() * material_pdf);

                    // Return... light sources have a pdf equal to their 1/Omega (i.e. their size)
                    ret += fx / denominator;
                } // end of unobstructed distance
            } // end of iterating samples
        } // end of iterating lights
          // return
        ret / total_samples as f64
    }

    /// Casts a [`Ray3D`] and returns an `Option<(f64,Vector3D,usize)>` in which the
    /// [`Vector3D`] is the normal at the point of intersection, the `usize`
    /// is the index of the [`Material`] encountered, and the `f64` is the distance to it.    
    pub fn cast_ray(&self, ray: &Ray3D) -> Option<(f64, Vector3D, usize)> {
        const MIN_T: f64 = 0.000001;

        let mut t = f64::MAX;
        let mut material_index = usize::MAX;
        let mut intersected = false;
        let mut normal = Vector3D::new(0., 0., 0.);

        for object in self.objects.iter() {
            if let Some(new_t) = object.primitive.intersect(&ray) {
                // Is it a valid hit and it is earlier than the rest?
                if t > MIN_T && new_t < t {
                    // Update info.
                    let (new_normal, new_surface_side) =
                        object.primitive.normal_at_intersection(ray, new_t);
                    t = new_t;
                    normal = new_normal;
                    material_index = match new_surface_side {
                        SurfaceSide::Front => object.front_material_index,
                        SurfaceSide::Back => object.back_material_index,
                    };
                    intersected = true;
                }
            }
        }

        // Return
        if !intersected {
            None
        } else {
            Some((t, normal, material_index))
        }
    }

    fn unobstructed_distance(&self, ray: &Ray3D, distance: f64) -> bool {
        const MIN_T: f64 = 0.000001;

        debug_assert!((1. - ray.direction.length()).abs() < 0.00000001);

        // Check all objects
        for object in self.objects.iter() {
            // If it intersects an object,
            if let Some(t) = object.primitive.intersect(&ray) {
                // Is it a valid hit and it is earlier than the rest?
                if t > MIN_T && t + MIN_T < distance && (distance - t).abs() > 0.0001 {
                    return false;
                }
            }
        }

        // it is unobstructed
        true
    }

    pub fn push_material(&mut self, material: Box<dyn Material>) -> usize {
        self.materials.push(material);
        // return
        self.materials.len() - 1
    }

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

        // Mark as source
        if self.materials[front_material_index].emits_direct_light()
            || self.materials[back_material_index].emits_direct_light()
        {
            self.lights.push(this_index)
        }
        // Push
        self.objects.push(Object {
            front_material_index,
            back_material_index,
            primitive: object,
        });
        // return
        this_index
    }

    /// Borrows a [`Material`]
    pub fn borrow_material(&self, i: usize) -> &Box<dyn Material> {
        &self.materials[i]
    }

    /// Borrows an [`Object`]
    pub fn borrow_object(&self, i: usize) -> &Box<dyn Sampleable> {
        &self.objects[i].primitive
    }

    /// Borrows a light
    pub fn light(&self, i:usize)->usize{
        self.lights[i]
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_shadow_ray() {
        assert!(false)
    }
}

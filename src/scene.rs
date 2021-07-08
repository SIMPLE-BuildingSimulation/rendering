use crate::material::Material;
use geometry3d::intersect_trait::{Intersect, SurfaceSide};
use geometry3d::ray3d::Ray3D;
use geometry3d::vector3d::Vector3D;

struct Object {
    primitive: Box<dyn Intersect>,
    front_material_index: usize,
    back_material_index: usize,
}

#[derive(Default)]
pub struct Scene {
    /// Objects in the scene that are not tested
    /// directly for shadow (e.g., non-luminous objects
    /// and diffuse light)
    objects: Vec<Object>,

    /// The materials in the scene
    materials: Vec<Box<dyn Material>>,

    /// The light sources in the scene that
    /// have to be tested directly
    lights: Vec<Object>,
}

impl Scene {
    /// Casts a [`Ray3D`] and returns an `Option<(f64,usize)>` in which the `usize`
    /// is the object index intersected, and the `f64` is the distance `t` to it.
    ///
    /// So far, this does not cast against light sources (sample those when
    /// shading)
    pub fn cast_ray(&self, ray: &Ray3D) -> Option<(f64, Vector3D, usize)> {
        const MIN_T: f64 = 0.000001;

        let mut t = f64::MAX;
        let mut material_index = usize::MAX;
        let mut intersected = false;
        let mut normal = Vector3D::new(0., 0., 0.);

        for (i, object) in self.objects.iter().enumerate() {
            if let Some((new_t, new_normal, new_surface_side)) = object.primitive.intersect(&ray) {
                // Is it a valid hit and it is earlier than the rest?
                if t > MIN_T && new_t < t {
                    // Update info.
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

    // pub fn lights(&self)->&Vec<Object>{
    //     &self.lights
    // }

    pub fn push_material(&mut self, material: Box<dyn Material>) -> usize {
        let ret = self.materials.len();
        self.materials.push(material);
        ret
    }

    pub fn push_object(
        &mut self,
        front_material_index: usize,
        back_material_index: usize,
        object: Box<dyn Intersect>,
    ) -> usize {
        let ret = self.materials.len();
        self.objects.push(Object {
            front_material_index,
            back_material_index,
            primitive: object,
        });
        ret
    }

    pub fn borrow_material(&self, i: usize) -> &Box<dyn Material> {
        &self.materials[i]
    }
}

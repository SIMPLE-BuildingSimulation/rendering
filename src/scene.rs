use crate::sampleable_trait::Sampleable;
use crate::material::Material;
use geometry3d::intersect_trait::SurfaceSide;
use geometry3d::point3d::Point3D;
use geometry3d::ray3d::Ray3D;
use geometry3d::vector3d::Vector3D;

use crate::colour::Spectrum;

struct Object {
    primitive: Box<dyn Sampleable>,
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
    lights: Vec<usize>,
}

impl Scene {
    pub fn get_local_illumination(&self, material:&Box<dyn Material>, vin:Vector3D, point: Point3D, normal: Vector3D) -> Spectrum {
        // prevent self-shading
        let origin = point + normal * 0.0001;
        let mut ret = Spectrum::black();

        for light_index in &self.lights {
            println!("... found light {}", light_index);
            let primitive = &self.objects[*light_index].primitive;
            let (light_distance,light_direction) = primitive.direction(point);

            // Expect direction to be normalized
            debug_assert!(( 1.-light_direction.length() ).abs()<0.0001);            
            
            let shadow_ray = Ray3D {
                origin,
                direction: light_direction,
            };

            // If the light is visible
            if self.unobstructed_distance(&shadow_ray, light_distance){
                println!("Unobsteucted");
                if let Some((.., side)) = primitive.intersect(&shadow_ray) {
                         
                    let cos_theta = (normal * light_direction).abs();

                    let light_material = match side {
                        SurfaceSide::Front => &self.materials[self.objects[*light_index].front_material_index],
                        SurfaceSide::Back =>  &self.materials[self.objects[*light_index].back_material_index],
                    };
                    let light_colour = light_material.colour();

                    
                    //let one_over_r2 = 1./light_distance/light_distance;
                    let one_over_r2 = 1.;
                    ret += light_colour * cos_theta * material.bsdf(vin, normal, shadow_ray.direction) * one_over_r2;

                }else{
                    unreachable!();// right?
                }                                
            }                    
        }
        // return
        ret
    }

    

    /// Casts a [`Ray3D`] and returns an `Option<(f64,Vector3D,usize)>` in which the 
    /// [`Vector3D`] is the normal at the point of intersection, the `usize`
    /// is the index of the [`Material`] encountered, and the `f64` is the distance to it.
    ///
    /// So far, this does not cast against light sources (sample those when
    /// shading)
    pub fn cast_ray(&self, ray: &Ray3D) -> Option<(f64, Vector3D, usize)> {
        const MIN_T: f64 = 0.000001;

        let mut t = f64::MAX;
        let mut material_index = usize::MAX;
        let mut intersected = false;
        let mut normal = Vector3D::new(0., 0., 0.);

        for object in self.objects.iter() {
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

    pub fn unobstructed_distance(&self, ray: &Ray3D, distance: f64) -> bool {
        const MIN_T: f64 = 0.000001;

        // Check all objects
        for object in self.objects.iter() {
            // If it intersects an object,
            if let Some((t, ..)) = object.primitive.intersect(&ray) {                
                // Is it a valid hit and it is earlier than the rest?
                if t > MIN_T && t + MIN_T < distance {
                    return false
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
        
        if front_material_index >= self.materials.len(){
            panic!("Pushing object with front material out of bounds")
        }

        if back_material_index >= self.materials.len(){
            panic!("Pushing object with back material out of bounds")
        }

        let this_index = self.objects.len();

        // Mark as source
        if self.materials[front_material_index].is_light_source()||self.materials[back_material_index].is_light_source(){
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

    pub fn borrow_material(&self, i: usize) -> &Box<dyn Material> {
        &self.materials[i]
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_shadow_ray() {
        assert!(false)
    }
}

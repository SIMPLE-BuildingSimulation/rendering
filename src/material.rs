use crate::colour::Spectrum;
use crate::samplers::cosine_weighted_sample_hemisphere;
use geometry3d::vector3d::Vector3D;

fn mirror_direction(vin: Vector3D, normal: Vector3D) -> Vector3D {
    debug_assert!((vin.length() - 1.).abs() < 100. * f64::EPSILON);
    debug_assert!((normal.length() - 1.).abs() < 100. * f64::EPSILON);
    let mut ret = vin - normal * (2. * (vin * normal));
    ret.normalize();
    ret
}

pub trait Material {
    /// Retrieves the Colour of the material. This will usually
    /// represent the values that will multiply the different
    /// elements of the [`Spectrum`]. E.g., the reflectance values.
    fn colour(&self) -> Spectrum;

    /// Should this material be tested for direct illumination?
    ///
    /// Defaults to `false`
    fn emits_direct_light(&self) -> bool {
        false
    }

    /// Should this material emits light
    ///
    /// Defaults to `false`
    fn emits_light(&self) -> bool {
        false
    }

    /// Gets the BSDF's value for a certain combination of Vin, Vout and Normal
    /// [`Vector3D`]s.
    fn bsdf(&self, vin: Vector3D, normal: Vector3D, vout: Vector3D) -> f64;

    /// Gets a sample associated to the bsdf
    fn sample_bsdf(&self, vout: Vector3D, normal: Vector3D) -> Vector3D;

    /// Does this material scatter (e.g., like [`Plastic`]) or does it
    /// only transmit/reflects specularly (e.g., like [`Mirror`])?
    ///
    /// Defaults to `false`
    fn specular_only(&self) -> bool {
        false
    }
}

pub struct Light {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}
impl Material for Light {
    fn colour(&self) -> Spectrum {
        Spectrum {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }

    fn emits_direct_light(&self) -> bool {
        true
    }

    fn emits_light(&self) -> bool {
        true
    }

    // Lights don't reflect...?
    fn bsdf(&self, _: Vector3D, _: Vector3D, _: Vector3D) -> f64 {
        0.0
    }

    fn sample_bsdf(&self, _vout: Vector3D, _normal: Vector3D) -> Vector3D {
        panic!("Trying to sample the BSDF of a Light material")
    }
}

pub struct Metal {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub specularity: f64,
    pub roughness: f64,
}

impl Material for Metal {
    fn colour(&self) -> Spectrum {
        Spectrum {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }

    // Assume lambertian, for now
    fn bsdf(&self, _: Vector3D, _: Vector3D, _: Vector3D) -> f64 {
        const ONE_OVER_PI: f64 = 1. / std::f64::consts::PI;
        ONE_OVER_PI
    }

    fn sample_bsdf(&self, _vout: Vector3D, normal: Vector3D) -> Vector3D {
        let mut rng = rand::thread_rng();
        cosine_weighted_sample_hemisphere(&mut rng, normal)
    }
}

pub struct Plastic {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub specularity: f64,
    pub roughness: f64,
}

impl Material for Plastic {
    fn colour(&self) -> Spectrum {
        Spectrum {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }

    // Assume lambertian, for now
    fn bsdf(&self, _: Vector3D, _: Vector3D, _: Vector3D) -> f64 {
        const ONE_OVER_PI: f64 = 1. / std::f64::consts::PI;
        ONE_OVER_PI
    }

    fn sample_bsdf(&self, _vout: Vector3D, normal: Vector3D) -> Vector3D {
        let mut rng = rand::thread_rng();
        cosine_weighted_sample_hemisphere(&mut rng, normal)
    }
}

pub struct Mirror {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Material for Mirror {
    fn colour(&self) -> Spectrum {
        Spectrum {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }

    fn bsdf(&self, vin: Vector3D, normal: Vector3D, vout: Vector3D) -> f64 {
        let mirror = mirror_direction(vin, normal);
        // All of it goes to the mirror direction
        if vout.is_parallel(mirror) {
            1.
        } else {
            0.
        }
    }

    fn sample_bsdf(&self, vout: Vector3D, normal: Vector3D) -> Vector3D {
        mirror_direction(vout, normal)
    }
}

// pub struct Dielectric{
//     pub red: f64,
//     pub green: f64,
//     pub blue: f64,
//     pub refraction_coefficient: f64,
// }

// impl Dielectric {

// }

// impl Material for Dielectric{
//     fn colour(&self) -> Spectrum {
//         Spectrum {
//             red: self.red,
//             green: self.green,
//             blue: self.blue,
//         }
//     }

//     fn bsdf(&self, vin: Vector3D, normal: Vector3D, vout: Vector3D) -> Spectrum{

//     }

//     fn specular_only(&self)->bool{
//         false
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirror_direction() {
        fn check(v: Vector3D, normal: Vector3D, mirror: Vector3D) -> Result<(), String> {
            let v = v.get_normalized();
            let normal = normal.get_normalized();
            let mirror = mirror.get_normalized();

            let found = mirror_direction(v, normal);
            if !(mirror - found).is_zero() {
                return Err(format!(
                    "Expected mirror direction was {} | found {}",
                    mirror, found
                ));
            }
            Ok(())
        }

        check(
            Vector3D::new(0., 0., 1.),
            Vector3D::new(0., 0., 1.),
            Vector3D::new(0., 0., -1.),
        )
        .unwrap();
        check(
            Vector3D::new(0., 0., -1.),
            Vector3D::new(0., 0., -1.),
            Vector3D::new(0., 0., 1.),
        )
        .unwrap();
        check(
            Vector3D::new(1., 0., -1.).get_normalized(),
            Vector3D::new(0., 0., 1.),
            Vector3D::new(1., 0., 1.),
        )
        .unwrap();
    }
}

use crate::colour::Spectrum;
use geometry3d::vector3d::Vector3D;

fn mirror_direction(vin: Vector3D, normal: Vector3D)->Vector3D{
    debug_assert!((vin.length() - 1.).abs()<100.*f64::EPSILON);
    debug_assert!((normal.length() - 1.).abs()<100.*f64::EPSILON);

    let mut ret = vin - normal * 2.;
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
    fn is_light_source(&self) -> bool {
        false
    }

    /// Gets the BSDF value for a certain combination of Vin, Vout and Normal
    /// [`Vector3D`]s.
    fn bsdf(&self, vin: Vector3D, normal: Vector3D, vout: Vector3D) -> Spectrum;

    // Does this material scatter (e.g., like [`Plastic`]) or does it
    // only transmit/reflects specularly (e.g., like [`Mirror`])?
    // 
    // Defaults to `false`
    // fn specular_only(&self)->bool{
    //     false
    // }
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

    fn is_light_source(&self) -> bool {
        true
    }

    // Lights don't reflect...?
    fn bsdf(&self, _: Vector3D, _: Vector3D, _: Vector3D) -> Spectrum {
        Spectrum::black()
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
    fn bsdf(&self, _: Vector3D, _: Vector3D, _: Vector3D) -> Spectrum {
        const ONE_OVER_PI: f64 = 1. / std::f64::consts::PI;
        self.colour() * ONE_OVER_PI
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
    fn bsdf(&self, _: Vector3D, _: Vector3D, _: Vector3D) -> Spectrum {
        const ONE_OVER_PI: f64 = 1. / std::f64::consts::PI;
        self.colour() * ONE_OVER_PI
    }
}


pub struct Mirror{
    pub red: f64,
    pub green: f64,
    pub blue: f64,    
}



impl Material for Mirror{
    fn colour(&self) -> Spectrum {
        Spectrum {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
    
    fn bsdf(&self, vin: Vector3D, normal: Vector3D, vout: Vector3D) -> Spectrum{
        let mirror = mirror_direction(vin, normal);
        // All of it goes to the mirror direction        
        if vout.is_same_direction(mirror){
            self.colour()
        }else{
            Spectrum::black()
        }
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
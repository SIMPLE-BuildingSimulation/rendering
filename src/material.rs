use crate::colour::Spectrum;
use geometry3d::vector3d::Vector3D;

pub trait Material {
    fn colour(&self)->Spectrum;

    fn is_light_source(&self)->bool{
        false
    }

    fn bsdf(&self, vin: Vector3D, normal:Vector3D, vout:Vector3D)->Spectrum;
}

pub struct Light {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}
impl Material for Light {
    
    fn colour(&self)->Spectrum{
        Spectrum{
            red: self.red,
            green: self.green,
            blue:self.blue,
        }
    }

    fn is_light_source(&self)->bool{
        true
    }

    // Lights don't reflect...?
    fn bsdf(&self, _: Vector3D, _:Vector3D, _:Vector3D)->Spectrum{
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
    fn colour(&self)->Spectrum{
        Spectrum{
            red: self.red,
            green: self.green,
            blue:self.blue,
        }
    }

    // Assume lambertian, for now
    fn bsdf(&self, _: Vector3D, _:Vector3D, _:Vector3D)->Spectrum{
        const ONE_OVER_PI : f64 = 1./std::f64::consts::PI;
        self.colour()*ONE_OVER_PI
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
    fn colour(&self)->Spectrum{
        Spectrum{
            red: self.red,
            green: self.green,
            blue:self.blue,
        }
    }

    // Assume lambertian, for now
    fn bsdf(&self, _: Vector3D, _:Vector3D, _:Vector3D)->Spectrum{
        const ONE_OVER_PI : f64 = 1./std::f64::consts::PI;
        self.colour()*ONE_OVER_PI
    }
}

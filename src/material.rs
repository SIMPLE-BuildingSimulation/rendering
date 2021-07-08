pub trait Material {
    fn red(&self) -> f64;
    fn green(&self) -> f64;
    fn blue(&self) -> f64;
}

pub struct Light {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}
impl Material for Light {
    fn red(&self) -> f64 {
        self.red
    }
    fn green(&self) -> f64 {
        self.green
    }
    fn blue(&self) -> f64 {
        self.blue
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
    fn red(&self) -> f64 {
        self.red
    }
    fn green(&self) -> f64 {
        self.green
    }
    fn blue(&self) -> f64 {
        self.blue
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
    fn red(&self) -> f64 {
        self.red
    }
    fn green(&self) -> f64 {
        self.green
    }
    fn blue(&self) -> f64 {
        self.blue
    }
}

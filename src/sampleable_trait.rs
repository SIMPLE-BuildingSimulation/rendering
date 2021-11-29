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

use rand::prelude::*;

use crate::{Float,PI};
use crate::samplers::*;
use geometry3d::intersect_trait::Intersect;
use geometry3d::{Point3D, Triangle3D, Vector3D, Sphere3D, DistantSource3D};


pub trait Sampleable: Intersect {
    /// Receives a [`Point3D`] and returns the distance `t`
    /// and a NORMALIZEd [`Vector3D`] pointing towards it
    fn direction(&self, point: Point3D) -> (Float, Vector3D);

    /// Produces an [`Iterator`] that produces samples (i.e., [`Vector3D`])
    /// pointing from the [`Point3D`] `p` towards the object.
    fn direction_sampler(
        &self,
        _point: Point3D,
        _n_samples: usize,
    ) -> Box<dyn Iterator<Item = Vector3D>> {
        unimplemented!()
    }

    /// Produces an [`Iterator`] that produces samples (i.e., [`Point3D`])
    /// located on top of the Sampleable object
    fn surface_sampler(&self, _n_samples: usize) -> Box<dyn Iterator<Item = Point3D>> {
        unimplemented!()
    }

    /// Returns the solid angle covered by a primitive
    /// as seen—unobstructed—from a certain [`Point3D`].
    fn omega(&self, point: Point3D) -> Float;

    // Samples the center of the
}

/* TRIANGLE */

pub struct TriangleDirectionSampler {    
    pub ray_origin: Point3D,    
    pub surface_sampler: TriangleSurfaceSampler,        
}

impl Iterator for TriangleDirectionSampler {
    type Item = Vector3D;
    fn next(&mut self) -> Option<Self::Item> {
        let target_p = self.surface_sampler.next()?;        
        let direction = target_p - self.ray_origin;                
        Some(direction.get_normalized())
    }
}


pub struct TriangleSurfaceSampler {    
    pub a: Point3D,
    pub b: Point3D,
    pub c: Point3D,
    pub n_samples: usize,
    pub i: usize,
    pub rng: ThreadRng,
}

impl Iterator for TriangleSurfaceSampler {
    type Item = Point3D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_samples == self.i {
            return None;
        }
        self.i += 1;
                
        // Get a direction on a unit Northern hemisphere        
        Some(uniform_sample_triangle(&mut self.rng, self.a, self.b, self.c))
    }
}


impl Sampleable for Triangle3D {
    fn direction(&self, point: Point3D) -> (Float, Vector3D) {
        const THIRD : Float = 1./3.;
        // Do the rest
        let centroid = (self.a() + self.b() + self.c()) * THIRD;
        let direction = centroid - point;
        let t = direction.length();
        (t, direction / t)
    }

    fn omega(&self, _point: Point3D) -> Float {
        unimplemented!();
        // let direction = self.outer_centroid() - point;
        // let t = direction.length_squared();
        // self.area() / t
    }

    fn direction_sampler(
        &self,
        point: Point3D,
        n_samples: usize,
    ) -> Box<dyn Iterator<Item = Vector3D>> {
        Box::new(TriangleDirectionSampler {
            ray_origin: point,
            surface_sampler: TriangleSurfaceSampler{
                a: self.a(),
                b: self.b(),
                c: self.c(),
                n_samples,
                i: 0,
                rng: rand::thread_rng(),
            }
        })
    }

    fn surface_sampler(&self, n_samples: usize) -> Box<dyn Iterator<Item = Point3D>> {
        Box::new(TriangleSurfaceSampler{
            a: self.a(),
            b: self.b(),
            c: self.c(),
            n_samples,
            i: 0,
            rng: rand::thread_rng(),
        })
    }
}
/* END TRIANGLE */



/* SPHERE */

pub struct InsideSphereDirectionSampler {    
    pub ray_origin: Point3D,    
    pub surface_sampler: SphereSurfaceSampler
}

impl Iterator for InsideSphereDirectionSampler {
    type Item = Vector3D;
    fn next(&mut self) -> Option<Self::Item> {
        let target_p = self.surface_sampler.next()?;
        let dir = target_p - self.ray_origin;
        Some(dir.get_normalized())
    }
}

pub struct SphereDirectionSampler {
    pub normal: Vector3D,
    pub ray_origin: Point3D,
    pub centre: Point3D,
    pub radius: Float,
    pub n_samples: usize,
    pub i: usize,
    pub rng: ThreadRng,
}

impl Iterator for SphereDirectionSampler {
    type Item = Vector3D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_samples == self.i {
            return None;
        }
        self.i += 1;
        let direction = self.centre - self.ray_origin;
        if direction.length() < self.radius {
            // if we are inside of the sphere
            panic!("Trying to sample sphere from inside it.")
        }

        if self.i == 0 {
            // First sample is center
            return Some(direction.get_normalized());
        }
        let p = uniform_sample_disc(&mut self.rng, self.radius, self.centre, self.normal);
        let direction = p - self.ray_origin;
        Some(direction.get_normalized())
    }
}


pub struct SphereSurfaceSampler {    
    pub centre: Point3D,
    pub radius: Float,
    pub n_samples: usize,
    pub i: usize,
    pub rng: ThreadRng,
}

impl Iterator for SphereSurfaceSampler {
    type Item = Point3D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_samples == self.i {
            return None;
        }
        self.i += 1;
                
        // Get a direction on a unit Northern hemisphere
        let mut vec = uniform_sample_hemisphere(&mut self.rng, Vector3D::new(1., 0., 0.), Vector3D::new(0., 1., 0.), Vector3D::new(0., 0., 1.));

        // 50% chance of flipping it
        let coin : Float = self.rng.gen();
        if coin > 0.5 {
            vec *= self.radius
        }else{
            vec *= -self.radius
        }
        
        let p = Point3D::new(vec.x, vec.y, vec.z) + self.centre;
        Some(p)
    }
}

impl Sampleable for Sphere3D {
    fn direction(&self, point: Point3D) -> (Float, Vector3D) {
        let direction = self.centre() - point;
        let t = direction.length();
        (t - self.radius, direction / t)
    }

    fn omega(&self, point: Point3D) -> Float {
        let d = (self.centre() - point).length();
        let d2 = d * d;
        PI * self.radius * self.radius / d2
    }

    fn direction_sampler(
        &self,
        point: Point3D,
        n_samples: usize,
    ) -> Box<dyn Iterator<Item = Vector3D>> {
        let centre = self.centre();
        let this_r_sqrd = (point - centre).length_squared();
        if this_r_sqrd > self.radius * self.radius{
            // if we are outside of the sphere
            Box::new(SphereDirectionSampler {
                n_samples,
                normal: (centre - point).get_normalized(),
                ray_origin: point,
                radius: self.radius,
                centre: centre,
                i: 0,
                rng: rand::thread_rng(),
            })
        }else{
            Box::new(InsideSphereDirectionSampler {    
                ray_origin: point,    
                surface_sampler: SphereSurfaceSampler{
                    centre: self.centre(),
                    radius: self.radius,
                    n_samples,
                    i: 0,
                    rng: rand::thread_rng(),
                }
            })
        }

    }

    fn surface_sampler(&self, n_samples: usize) -> Box<dyn Iterator<Item = Point3D>> {
        Box::new(SphereSurfaceSampler {    
            centre: self.centre(),
            radius: self.radius,
            n_samples,
            i: 0,
            rng: rand::thread_rng(),
        })
    }
}
/* END SPHERE */



/* DISTANT SOURCE */

pub struct DistantSourceSampler {
    pub normal: Vector3D,
    pub ray_origin: Point3D,
    pub centre: Point3D,
    pub radius: Float,
    pub n_samples: usize,
    pub i: usize,
    pub rng: ThreadRng,
}

impl Iterator for DistantSourceSampler {
    type Item = Vector3D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_samples == self.i {
            return None;
        }
        self.i += 1;
        let direction = self.centre - self.ray_origin;
        if self.i == 0 {
            // First sample is center
            return Some(direction.get_normalized());
        }

        let p = uniform_sample_disc(&mut self.rng, self.radius, self.centre, self.normal);
        let direction = p - self.ray_origin;

        Some(direction.get_normalized())
    }
}

impl Sampleable for DistantSource3D {
    /// It is always in the same direction
    fn direction(&self, _point: Point3D) -> (Float, Vector3D) {
        (Float::MAX - 1., self.direction)
    }

    fn omega(&self, _point: Point3D) -> Float {
        self.omega
    }

    fn direction_sampler(
        &self,
        point: Point3D,
        n_samples: usize,
    ) -> Box<dyn Iterator<Item = Vector3D>> {
        let normal = self.direction.get_normalized();
        let radius = (self.angle / 2.0).tan();
        Box::new(DistantSourceSampler {
            n_samples,
            normal,
            radius,
            ray_origin: point,
            centre: point + normal,
            i: 0,
            rng: rand::thread_rng(),
        })
    }
}


/* END DISTANT SOURCE */



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sphere_direction() {
        let p = Point3D::new(0., 0., 0.);
        let x = 0.1;
        let y = 10.23;
        let z = 38.1;
        let r = 1.2;
        let sphere = Sphere3D::new(r, Point3D::new(x, y, z));
        let (t, direction) = sphere.direction(p);

        assert!((t - (x * x + y * y + z * z).sqrt() + r).abs() < 0.000001);
        assert_eq!(Vector3D::new(x, y, z).get_normalized(), direction);
    }
}

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

// use rand::prelude::*;

use crate::rand::*;

use crate::{Float,PI};
use crate::samplers::*;
// use geometry3d::intersect_trait::Intersect;
use geometry3d::{Point3D, Triangle3D, Vector3D, Sphere3D, DistantSource3D};

#[cfg(feature = "parallel")]
pub trait SampleableRequirements:  Sync + Send {}

#[cfg(feature = "parallel")]
impl<T: Intersect + Sync + Send> SampleableRequirements for T {}

#[cfg(not(feature = "parallel"))]
pub trait SampleableRequirements {}

#[cfg(not(feature = "parallel"))]
impl<T> SampleableRequirements for T {}


pub trait Sampleable : SampleableRequirements {
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
    pub rng: RandGen,
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


pub fn triangle_direction_sampler(
    triangle: &Triangle3D,
    point: Point3D,
    n_samples: usize,
) -> Box<dyn Iterator<Item = Vector3D>> {
    Box::new(TriangleDirectionSampler {
        ray_origin: point,
        surface_sampler: TriangleSurfaceSampler{
            a: triangle.a(),
            b: triangle.b(),
            c: triangle.c(),
            n_samples,
            i: 0,
            rng: get_rng(),
        }
    })
}

pub fn triangle_direction(triangle: &Triangle3D, point: Point3D) -> (Float, Vector3D) {
    const THIRD : Float = 1./3.;
    // Do the rest
    let centroid = (triangle.a() + triangle.b() + triangle.c()) * THIRD;
    let direction = centroid - point;
    let t = direction.length();
    (t, direction / t)
}

pub fn triangle_omega(_triangle: &Triangle3D, _point: Point3D) -> Float {
    unimplemented!();
    // let direction = self.outer_centroid() - point;
    // let t = direction.length_squared();
    // self.area() / t
}

pub fn triangle_surface_sampler(triangle: &Triangle3D, n_samples: usize) -> Box<dyn Iterator<Item = Point3D>> {
    Box::new(TriangleSurfaceSampler{
        a: triangle.a(),
        b: triangle.b(),
        c: triangle.c(),
        n_samples,
        i: 0,
        rng: get_rng(),
    })
}

/* END OF TRIANGLE */



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
    pub rng: RandGen,
}

impl Iterator for SphereDirectionSampler {
    type Item = Vector3D;
    fn next(&mut self) -> Option<Self::Item> {
        if self.n_samples == self.i {
            return None;
        }
        self.i += 1;
        let r2 = self.radius * self.radius;
        let direction = self.centre - self.ray_origin;
        if direction.length_squared() <= r2 {
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
    pub rng: RandGen,
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

pub fn sphere_direction(sphere: &Sphere3D, point: Point3D) -> (Float, Vector3D) {
    let direction = sphere.centre() - point;
    let t = direction.length();
    (t - sphere.radius, direction / t)
}

pub fn sphere_omega(sphere: &Sphere3D, point: Point3D) -> Float {
    let d = (sphere.centre() - point).length();
    let d2 = d * d;
    PI * sphere.radius * sphere.radius / d2
}

pub fn sphere_direction_sampler(
    sphere: &Sphere3D,
    point: Point3D,
    n_samples: usize,
) -> Box<dyn Iterator<Item = Vector3D>> {
    let centre = sphere.centre();
    let this_r_sqrd = (point - centre).length_squared();
    if this_r_sqrd > sphere.radius * sphere.radius{
        debug_assert!((centre - point).length_squared() >= 1e-9);
        // if we are outside of the sphere
        Box::new(SphereDirectionSampler {
            n_samples,
            normal: (centre - point).get_normalized(),
            ray_origin: point,
            radius: sphere.radius,
            centre: centre,
            i: 0,
            rng: get_rng(),
        })
    }else{
        Box::new(InsideSphereDirectionSampler {    
            ray_origin: point,    
            surface_sampler: SphereSurfaceSampler{
                centre: sphere.centre(),
                radius: sphere.radius,
                n_samples,
                i: 0,
                rng: get_rng(),
            }
        })
    }

}

pub fn sphere_surface_sampler(sphere: &Sphere3D, n_samples: usize) -> Box<dyn Iterator<Item = Point3D>> {
    Box::new(SphereSurfaceSampler {    
        centre: sphere.centre(),
        radius: sphere.radius,
        n_samples,
        i: 0,
        rng: get_rng(),
    })
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
    pub rng: RandGen,
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

/// It is always in the same direction
pub fn source_direction(source: &DistantSource3D, _point: Point3D) -> (Float, Vector3D) {
    (Float::MAX - 1., source.direction)
}

pub fn source_omega(source: &DistantSource3D, _point: Point3D) -> Float {
    source.omega
}

pub fn source_direction_sampler(
    source: &DistantSource3D,
    point: Point3D,
    n_samples: usize,
) -> Box<dyn Iterator<Item = Vector3D>> {
    let normal = source.direction.get_normalized();
    let radius = (source.angle / 2.0).tan();
    Box::new(DistantSourceSampler {
        n_samples,
        normal,
        radius,
        ray_origin: point,
        centre: point + normal,
        i: 0,
        rng: get_rng(),
    })
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
        let (t, direction) = sphere_direction(&sphere, p);

        assert!((t - (x * x + y * y + z * z).sqrt() + r).abs() < 0.000001);
        assert_eq!(Vector3D::new(x, y, z).get_normalized(), direction);
    }
}

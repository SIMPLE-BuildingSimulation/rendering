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

use crate::Float;

use geometry3d::{
    Triangle3D,
    Sphere3D,    
    Cylinder3D,
    DistantSource3D,
    BBox3D,      
    Ray3D,  
    Point3D,
    Vector3D,
};

use geometry3d::intersection::IntersectionInfo;
use crate::primitive_samplers::*;

#[derive(Clone)]
pub enum Primitive{
    Sphere(Sphere3D),
    Triangle(Triangle3D),
    Cylinder(Cylinder3D),
    Source(DistantSource3D)
}

impl Primitive{

    /// The name of the `Primitive`. Useful for debugging.
    pub fn id(&self) -> &'static str {
        match self {
            Self::Sphere(s)=>s.id(),
            Self::Triangle(s)=>s.id(),
            Self::Cylinder(s)=>s.id(),
            Self::Source(s)=>s.id(),
        }
    }

    /// Gets a `BBox3D` bounding the primitive, in world's coordinates.
    pub fn world_bounds(&self)->BBox3D{
        match self {
            Self::Sphere(s)=>s.world_bounds(),
            Self::Triangle(s)=>s.world_bounds(),
            Self::Cylinder(s)=>s.world_bounds(),
            Self::Source(_)=>panic!("Trying to get the bounds of a DistantSource3D"),
        }
    }


    /// Intersects an object with a [`Ray3D]` (IN WORLD COORDINATES) traveling forward, returning the distance
    /// `t` and the normal [`Vector3D`] at that point. If the distance
    /// is negative (i.e., the object is behind the plane), it should return
    /// [`None`]. Returns detailed [`IntersectionInfo`] about the intersaction .    
    pub fn intersect(&self, ray: &Ray3D) -> Option<IntersectionInfo> {
        match self {
            Self::Sphere(s)=>s.intersect(ray),
            Self::Triangle(s)=>s.intersect(ray),
            Self::Cylinder(s)=>s.intersect(ray),
            Self::Source(s)=>s.intersect(ray),
        }
    }

    /// Intersects an object with a [`Ray3D]` (IN WORLD COORDINATES) traveling forward, returning the distance
    /// `t` and the normal [`Vector3D`] at that point. If the distance
    /// is negative (i.e., the object is behind the plane), it should return
    /// [`None`]. Returns only the point of intersection.
    pub fn simple_intersect(&self, ray: &Ray3D) -> Option<Point3D> {
        match self {
            Self::Sphere(s)=>s.simple_intersect(ray),
            Self::Triangle(s)=>s.simple_intersect(ray),
            Self::Cylinder(s)=>s.simple_intersect(ray),
            Self::Source(s)=>s.simple_intersect(ray),
        }
    }

    pub fn omega(&self, point: Point3D) -> Float {
        match self {
            Self::Sphere(s)=>sphere_omega(s,point),
            Self::Triangle(s)=>triangle_omega(s, point),
            Self::Cylinder(_s)=>unimplemented!(),
            Self::Source(s)=>source_omega(s, point),
        }
    }

    pub fn direction(&self, point: Point3D) -> (Float, Vector3D) {
        match self {
            Self::Sphere(s)=>sphere_direction(s, point),
            Self::Triangle(s)=>triangle_direction(s, point),
            Self::Cylinder(_s)=>unimplemented!(),
            Self::Source(s)=>source_direction(s, point),
        }
    }

    pub fn direction_sampler(
        &self,
        point: Point3D,
        n_samples: usize,
    ) -> Box<dyn Iterator<Item = Vector3D>> {
        match self {
            Self::Sphere(s)=>sphere_direction_sampler(s, point, n_samples),
            Self::Triangle(s)=>triangle_direction_sampler(&s, point, n_samples),
            Self::Cylinder(_s)=>unimplemented!(),
            Self::Source(s)=>source_direction_sampler(s, point, n_samples),
        }
    }
}
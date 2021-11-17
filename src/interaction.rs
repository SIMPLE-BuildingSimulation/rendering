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

use std::rc::Rc;
use crate::Float;
use geometry3d::{Point3D, Vector3D, Transform};
use geometry3d::intersect_trait::SurfaceSide;

use crate::scene::Object;


#[derive(Clone,Copy)]
pub struct ShadingInfo{
    /// The normal [`Vector3D`] at the interaction.
    /// Must have the value of (dpdu x dpdv).normalize() ... use
    /// ShadingInfo::new(..) and this is done automatically
    pub normal : Vector3D,

    /// Partial derivative of the position `p` with respect to `u`
    pub dpdu: Vector3D, 

    /// Partial derivative of the position `p` with respect to `v`
    pub dpdv: Vector3D, 

    /// Partial derivative of the normal `n` with respect to u
    pub dndu: Vector3D,  

    /// Partial derivative of the normal `n` with respect to v
    pub dndv: Vector3D, 

    pub u: Float,
    pub v: Float,
    pub side: SurfaceSide,
}

impl ShadingInfo{
    pub fn new(u: Float, v: Float, dpdu: Vector3D, dpdv: Vector3D, dndu: Vector3D, dndv: Vector3D, side: SurfaceSide)->Self{
        debug_assert!(u>=0.);
        debug_assert!(u<=1.);
        debug_assert!(v>=0.);
        debug_assert!(v<=1.);
        let normal = dpdu.cross(dpdv).get_normalized();
        Self{
            dpdv,dpdu,dndu,dndv,normal, u, v, side
        }        
    }

    pub fn transform(&self, t: &Transform)->Self{
        Self{
            u: self.u,
            v: self.v,
            normal: t.transform_normal(self.normal),
            dpdu: t.transform_vec(self.dpdu), 
            dpdv: t.transform_vec(self.dpdv), 
            dndu: t.transform_vec(self.dndu), 
            dndv: t.transform_vec(self.dndv), 
            side: self.side
        }
    }
}


/// The data for a SurfaceInteraction
pub struct SurfaceInteractionData{
    /* GENERAL INTERACTION DATA */

    /// The [`Point3D`] of the interaction
    pub point: Point3D,

    // The floating point error at the intersection
    // pub perror: Point3D,

    /// The time of the intersection
    pub time: Float,

    /// The outgoing direction at the interaction. 
    /// This is the negative ray direction
    pub wo: Vector3D,

    // Scaterring media at the point of interaction
    // pub medium_interface: MediumInterface,

    /* FOR SURFACE INTERACTION */

    /// Stores the shading information based on
    /// pure geometry
    pub geometry_shading: ShadingInfo,

    /// Stores the shading information after being 
    /// perturbed by a texture
    pub texture_shading: Option<ShadingInfo>,

    /// The [`Object`] in the scene 
    pub object: Rc<Object>,    
}


impl SurfaceInteractionData {
    pub fn transform(&self, t: &Transform)->Self{
        // let (point, perror) = t.transform_pt_propagate_error(self.point, self.perror);
        let point = t.transform_pt(self.point);
        let wo = t.transform_vec(self.wo);
        let time = self.time;
        // let object = self.object;

        // shading
        let geometry_shading = self.geometry_shading.transform(t);
        let texture_shading = match self.texture_shading{
            Some(s)=>Some(s.transform(t)),
            None=>None
        };        
        Self{
            point,
            // perror,
            wo,
            time,
            geometry_shading,
            texture_shading,            
            object: Rc::clone(&self.object)
        }
    }

    /// Retrieves the normal of the [`SurfaceInteractionData`].
    /// Prioritizes the texture geometry (which can deviate the normal).
    /// If there is `None`, then the geometry shading is used.
    pub fn normal(&self)->Vector3D{
        match &self.texture_shading{
            Some(info)=>info.normal,
            None => self.geometry_shading.normal
        }
    }
}


pub enum Interaction{
    Surface(SurfaceInteractionData)    
}

impl std::fmt::Debug for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Surface(_)=>{
                f.debug_struct("SurfaceInteraction(..)").finish()
            },            
        }
        
    }
}

impl Interaction{

    pub fn new_surface(data: SurfaceInteractionData, flip_normal: bool)->Self{
        let mut data = data;
        if flip_normal{
            data.geometry_shading.normal *= -1.
        }
        Self::Surface(data)
    }

    /// Checks whether an [`Interaction`] is Surface
    pub fn is_surface_interaction(&self)->bool{
        match self{
            Self::Surface(_) => true,
            _ => false
        }
    }


    pub fn normal(&self)->Vector3D{
        match self{
            Self::Surface(d)=>d.normal(),
            _ => panic!("{:?} has no normals", self)
        }
    }

    pub fn object(&self)->&Rc<Object>{
        match self{
            Self::Surface(d)=>&d.object,
            _ => panic!("{:?} has no normals", self)
        }
    }
}
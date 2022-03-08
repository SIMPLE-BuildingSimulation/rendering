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
use crate::RefCount;
use geometry3d::intersection::SurfaceSide;
use geometry3d::{Point3D, Transform, Vector3D};

use crate::scene::Object;

#[derive(Clone, Copy)]
pub struct ShadingInfo {
    /// The normal [`Vector3D`] at the interaction.
    /// Must have the value of (dpdu x dpdv).normalize() ... use
    /// ShadingInfo::new(..) and this is done automatically
    pub normal: Vector3D,

    /// Partial derivative of the position `p` with respect to `u`
    pub dpdu: Vector3D,

    /// Partial derivative of the position `p` with respect to `v`
    pub dpdv: Vector3D,

    /// Partial derivative of the normal `n` with respect to u
    #[cfg(feature="textures")]
    pub dndu: Vector3D,

    /// Partial derivative of the normal `n` with respect to v
    #[cfg(feature="textures")]
    pub dndv: Vector3D,

    #[cfg(feature="textures")]
    pub u: Float,

    #[cfg(feature="textures")]
    pub v: Float,
    
    pub side: SurfaceSide,
}

impl ShadingInfo {
    pub fn new(
        _u: Float,
        _v: Float,
        dpdu: Vector3D,
        dpdv: Vector3D,
        _dndu: Vector3D,
        _dndv: Vector3D,
        side: SurfaceSide,
    ) -> Self {
        #[cfg(feature="textures")]
        {
            debug_assert!(_u >= 0.);
            debug_assert!(_u <= 1.);
            debug_assert!(_v >= 0.);
            debug_assert!(_v <= 1.);
        }
        let normal = dpdu.cross(dpdv).get_normalized();
        Self {
            dpdv,
            dpdu,
            side,
            normal,

            #[cfg(feature="textures")]
            dndu: _dndu,
            #[cfg(feature="textures")]
            dndv: _dndv,
            #[cfg(feature="textures")]
            u: _u,
            #[cfg(feature="textures")]
            v: _v,
        }
    }

    pub fn transform(&self, t: &Transform) -> Self {
        Self {
            normal: t.transform_normal(self.normal),
            dpdu: t.transform_vec(self.dpdu),
            dpdv: t.transform_vec(self.dpdv),
            side: self.side,

            #[cfg(feature="textures")]
            u: self.u,
            #[cfg(feature="textures")]
            v: self.v,
            #[cfg(feature="textures")]
            dndu: t.transform_vec(self.dndu),
            #[cfg(feature="textures")]
            dndv: t.transform_vec(self.dndv),
        }
    }
}

/// The data for a SurfaceInteraction
pub struct SurfaceInteractionData {
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
    #[cfg(feature="textures")]
    pub texture_shading: Option<ShadingInfo>,

    // /// The [`Object`] in the scene
    // pub object: RefCount<Object>,
    /// The index of the primitive in the primitives array
    pub prim_index: usize,
}

impl SurfaceInteractionData {
    pub fn transform(&self, t: &Transform) -> Self {
        // let (point, perror) = t.transform_pt_propagate_error(self.point, self.perror);
        let point = t.transform_pt(self.point);
        let wo = t.transform_vec(self.wo);
        let time = self.time;
        // let object = self.object;

        // shading
        let geometry_shading = self.geometry_shading.transform(t);        
        #[cfg(feature="textures")]
        let texture_shading = self.texture_shading.map(|s| s.transform(t));

        Self {
            point,
            // perror,
            wo,
            time,
            geometry_shading,

            #[cfg(feature="textures")]
            texture_shading,
            prim_index: self.prim_index,
        }
    }

    /// Retrieves the normal of the [`SurfaceInteractionData`].
    /// Prioritizes the texture geometry (which can deviate the normal).
    /// If there is `None`, then the geometry shading is used.
    pub fn normal(&self) -> Vector3D {
        #[cfg(feature="textures")]
        match &self.texture_shading {
            Some(info) => info.normal,
            None => self.geometry_shading.normal,
        }
        #[cfg(not(feature="textures"))]
        self.geometry_shading.normal
    }
}

pub enum Interaction {
    Surface(SurfaceInteractionData),    
    Endpoint(Option<RefCount<Object>>),
}

impl std::fmt::Debug for Interaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Surface(_) => f.debug_struct("SurfaceInteraction(..)").finish(),
            Self::Endpoint(light) => {
                let kind = if light.is_some() { "Light" } else { "Camera" };
                f.debug_struct(&format!("EndpointInteraction({})", kind))
                    .finish()
            }
        }
    }
}

impl Interaction {
    pub fn new_surface(data: SurfaceInteractionData, flip_normal: bool) -> Self {
        let mut data = data;
        if flip_normal {
            data.geometry_shading.normal *= -1.
        }
        Self::Surface(data)
    }

    /// Checks whether an [`Interaction`] is Surface
    pub fn is_surface_interaction(&self) -> bool {
        // match self{
        //     Self::Surface(_) => true,
        //     _ => false
        // }
        matches!(self, Self::Surface(_))
    }

    pub fn normal(&self) -> Vector3D {
        match self {
            Self::Surface(d) => d.normal(),
            _ => panic!("{:?} has no normals", self),
        }
    }

    // pub fn object(&self)->&RefCount<Object>{
    //     match self{
    //         Self::Surface(d)=>&d.object,
    //         _ => panic!("{:?} has no normals", self)
    //     }
    // }
}

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

use geometry3d::intersection::IntersectionInfo;
use geometry3d::{Point3D, Transform, Vector3D};

/// The data for a SurfaceInteraction]
#[derive(Default, Clone, Copy)]
pub struct Interaction {
    /* GENERAL INTERACTION DATA */
    /// The [`Point3D`] of the interaction
    pub point: Point3D,

    /// The outgoing direction at the interaction.
    /// This is the negative ray direction
    pub wo: Vector3D,

    // Scaterring media at the point of interaction
    // pub medium_interface: MediumInterface,

    /* FOR SURFACE INTERACTION */
    /// Stores the shading information based on
    /// pure geometry
    pub geometry_shading: IntersectionInfo,

    /// Stores the shading information after being
    /// perturbed by a texture
    #[cfg(feature = "textures")]
    pub texture_shading: Option<IntersectionInfo>,

    // /// The [`Object`] in the scene
    // pub object: RefCount<Object>,
    /// The index of the primitive in the primitives array
    pub prim_index: usize,
}

impl Interaction {
    pub fn transform(&self, t: &Transform) -> Self {
        // let (point, perror) = t.transform_pt_propagate_error(self.point, self.perror);
        let point = t.transform_pt(self.point);
        let wo = t.transform_vec(self.wo);

        // shading
        let geometry_shading = self.geometry_shading.transform(t);

        #[cfg(feature = "textures")]
        let texture_shading = self.texture_shading.as_ref().map(|s| s.transform(t));

        Self {
            point,
            wo,
            geometry_shading,
            prim_index: self.prim_index,

            #[cfg(feature = "textures")]
            texture_shading,
        }
    }

    /// Retrieves the normal of the [`SurfaceInteractionData`].
    /// Prioritizes the texture geometry (which can deviate the normal).
    /// If there is `None`, then the geometry shading is used.
    pub fn normal(&self) -> Vector3D {
        #[cfg(feature = "textures")]
        match &self.texture_shading {
            Some(info) => info.normal,
            None => self.geometry_shading.normal,
        }
        #[cfg(not(feature = "textures"))]
        self.geometry_shading.normal
    }
}

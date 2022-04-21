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
}

impl Interaction {
    pub fn transform(&self, t: &Transform) -> Self {
        // let (point, perror) = t.transform_pt_propagate_error(self.point, self.perror);
        let point = t.transform_pt(self.point);
        let wo = t.transform_vec(self.wo);

        // shading
        let geometry_shading = self.geometry_shading.transform(t);

        Self {
            point,
            wo,
            geometry_shading,
        }
    }
}

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

use crate::colour::Spectrum;
use crate::interaction::Interaction;
use crate::rand::*;
use crate::Float;
use geometry3d::{Point3D, Ray3D, Transform, Vector3D};

/// Represents a ray (of light?) beyond pure geometry. It
/// includes also the current index of refraction and, potentially,
/// time (for blurry images)
#[derive(Clone, Copy)]
pub struct Ray {
    /// Direction and position
    pub geometry: Ray3D,

    /// index of refraction of the current medium
    pub refraction_index: Float,

    /// Contains the information about the last hit.
    pub interaction: Interaction,

    pub depth: usize,

    pub value: Float,

    pub colour: Spectrum<{ crate::N_CHANELS }>,
}

impl std::default::Default for Ray {
    fn default() -> Self {
        Self {
            geometry: Ray3D {
                origin: Point3D::new(0., 0., 0.),
                direction: Vector3D::new(0., 0., 0.),
            },
            refraction_index: 1.,
            interaction: Interaction::default(),
            depth: 0,
            value: 1.,
            colour: Spectrum::<{ crate::N_CHANELS }>::ONE,
        }
    }
}

impl Ray {
    pub fn transform(&mut self, t: &Transform) {
        let (geometry, _o_error, _d_error) = t.transform_ray(&self.geometry);
        self.geometry = geometry;
    }

    pub fn direction(&self) -> Vector3D {
        self.geometry.direction
    }

    pub fn origin(&self) -> Point3D {
        self.geometry.origin
    }

    /// Returns the Intersection point, Normal, e1, e2
    pub fn get_triad(&self) -> (Point3D, Vector3D, Vector3D, Vector3D) {
        let intersection_pt = self.interaction.point;
        let normal = self.interaction.geometry_shading.normal;
        let e1 = self.interaction.geometry_shading.dpdu;
        let e2 = self.interaction.geometry_shading.dpdv;
        (intersection_pt, normal, e1, e2)
    }

    /// Get
    pub fn get_n_ambient_samples(
        &mut self,
        max_ambient_samples: usize,
        max_depth: usize,
        limit_weight: Float,
        rng: &mut RandGen,
    ) -> usize {
        if max_depth == 0 || max_ambient_samples <= 0 {
            0 // No ambient samples required
        } else if self.depth == 0 {
            max_ambient_samples
        } else {
            /* Adapted From Radiance's samp_hemi() at src/rt/ambcomp.c */
            /*


            if (ambacc <= FTINY && wt > (d = 0.8*intens(rcol)*r->rweight/(ambdiv*minweight)))
                wt = d;			/* avoid ray termination */
            n = sqrt(ambdiv * wt) + 0.5;
            i = 1 + (MINADIV-1)*(ambacc > FTINY);
            if (n < i)			/* use minimum number of samples? */
                n = i;

            // Improve readability, assuming that ambacc == 0.0, always (we don't have ambient cache here)

            d = 0.8*intens(rcol)*r->rweight/(ambdiv*minweight)
            if ( wt > d){
                wt = d;			/* avoid ray termination */
            }
            n = sqrt(ambdiv * wt) + 0.5;
            i = 1 + (MINADIV-1);
            if (n < i)			/* use minimum number of samples? */
                n = i;
            */
            let wt = self.value; //self.colour.radiance();

            // russian roullete
            let r: Float = rng.gen();
            if r > wt / limit_weight {
                self.value = limit_weight;
                return 0; // kill it!
            }
            1 // Stephen, this is on you.

            // let d = 0.8 * wt * self.colour.max() / (max_ambient_samples as Float * limit_weight);
            // if wt > d {
            //     wt = d;
            // }
            // let n = ((max_ambient_samples as Float * wt).sqrt() + 0.5).round() as usize;
            // const MIN_AMBS: usize = 1;
            // if n < MIN_AMBS {
            //     MIN_AMBS
            // } else {
            //     n
            // }
        }
    }
}

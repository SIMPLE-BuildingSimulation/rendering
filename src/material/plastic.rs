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
use crate::rand::*;
use crate::ray::Ray;
use crate::Float;
use geometry3d::{Point3D, Vector3D};

/// Information required for modelling Radiance's Plastic and Plastic
pub struct Plastic {
    pub colour: Spectrum<{ crate::N_CHANNELS }>,
    pub specularity: Float,
    pub roughness: Float,
}

impl Plastic {
    pub fn id(&self) -> &str {
        "Plastic"
    }

    pub fn colour(&self) -> Spectrum<{ crate::N_CHANNELS }> {
        self.colour
    }

    pub fn sample_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        intersection_pt: Point3D,
        ray: &mut Ray,
        rng: &mut RandGen,
    ) -> (Spectrum<{ crate::N_CHANNELS }>, Float) {
        let (direct, diffuse, weight) = crate::material::ward::sample_ward_anisotropic(
            normal,
            e1,
            e2,
            intersection_pt,
            self.specularity,
            self.roughness,
            self.roughness,
            ray,
            rng,
        );

        let bsdf = Spectrum::<{ crate::N_CHANNELS }>::gray(direct) + self.colour * diffuse;

        (bsdf, weight)
    }

    pub fn eval_bsdf(
        &self,
        normal: Vector3D,
        e1: Vector3D,
        e2: Vector3D,
        ray: &Ray,
        vout: Vector3D,
    ) -> Spectrum<{ crate::N_CHANNELS }> {
        let vout = vout * -1.;
        let (direct, diffuse) = crate::material::ward::evaluate_ward_anisotropic(
            normal,
            e1,
            e2,
            self.specularity,
            self.roughness,
            self.roughness,
            ray,
            vout,
        );

        Spectrum::<{ crate::N_CHANNELS }>::gray(direct) + self.colour * diffuse
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::colour::Spectrum;
    use geometry3d::Ray3D;
    use obj::raw::object::Point;

    // struct RadianceRayResult{
    //     pub depth: i32,
    //     pub origin: Point3D,
    //     pub intersection_pt: Point3D,
    //     pub normal: Vector3D,
    //     pub value: Spectrum::<3>,
    //     pub children: Vec<Box<Self>>,
    // }

    // /// Loads the stack of rays produced by running a Radiance command
    // /// as follows:
    // /// `echo 2 1 1 0 1 0 | rtrace -otopnv -h $RTRACE_OPTIONS $OCTREE  > results.txt`
    // fn load_ray_trace(filename: &str)->Box<RadianceRayResult>{
    //     let s = std::fs::read_to_string(filename).unwrap();

    //     fn ray_depth(line: &str)->i32{
    //         let mut level = 0;
    //         let mut start = String::new();
    //         while line.starts_with(&start){
    //             level += 1;
    //             start = format!("\t{start}");
    //         }

    //         level
    //     }

    //     fn parse_ray<'a>( v: &mut Vec<Box<RadianceRayResult>>, parent_depth: i32, index: &mut usize, lines :&Vec<&str>) {

    //         assert!(*index < lines.len());

    //         // Process current line
    //         let line = lines[*index];
    //         let depth = ray_depth(line);
    //         let a: Vec<f64> = line
    //             .split_ascii_whitespace()
    //             .into_iter()
    //             .map(|x| x.parse::<Float>().unwrap() )
    //             .collect();

    //         let origin = Point3D::new(a[0], a[1], a[2]);
    //         let intersection_pt = Point3D::new(a[3], a[4], a[5]);
    //         let normal = Vector3D::new(a[6], a[7], a[8]);
    //         let value = Spectrum([a[9],a[10], a[11]]);

    //         let l = v.len();
    //         let this = Box::new( RadianceRayResult {
    //             depth,
    //             origin,
    //             intersection_pt,
    //             normal,
    //             value,
    //             children: Vec::with_capacity(9),
    //         });

    //         if depth == parent_depth + 1 {
    //             // child.
    //             v.push( this);
    //         }else if depth

    //         // Check if there is anything else to do
    //         if *index+1 < lines.len(){
    //             *index += 1;
    //             // let next_depth = ray_depth(lines[*index]);

    //             if depth > parent_depth {
    //                 // new ray is child... parse next one into this one's children, at a greater depth
    //                 parse_ray(&mut ret, index, lines);
    //             }
    //         }

    //         // Nothing else to check. Return
    //         // ret

    //     }

    //     let lines : Vec<&str> = s.lines().rev().map(|x| x).collect();
    //     let mut index = 1;

    //     let mut ret = Vec::new();
    //     parse_ray ( &mut ret, &mut index, &lines);
    //     ret

    // }

    /// This test was developed by debugging RPICT in Radiance
    #[test]
    fn test_eval_plastic() {
        let plastic = Plastic {
            colour: Spectrum::<3>::from(0.5),
            specularity: 0.05,
            roughness: 0.1,
        };

        let origin = Point3D::new(2., 1., 1.);
        let direction = Vector3D::new(-0.446877862357762, 0.77495819368141017, 0.4469227832418069);
        let normal = Vector3D::new(1., 0., 0.);
        let vout = Vector3D::new(0.446877862357762, 0.77495819368141017, 0.4469227832418069);
        let ray = &Ray {
            geometry: Ray3D { origin, direction },
            .. Ray::default()
        };
        let e1 = Vector3D::new(0., 1., 0.);
        let e2 = Vector3D::new(0., 0., 1.);

        // alpha2	double	0.0099999999988358463
        // pdot	double	0.446877862357762
        // rdiff	double	0.95000000000291041
        // rspec	double	0.049999999997089616
        // scolor = [0.5, 0.5, 0.5] in ln 295
        // vrefl	FVECT
        // [0]	double	0.446877862357762
        // [1]	double	0.77495819368141017
        // [2]	double	0.4469227832418069
        // End of source.c / Direct
        // rcol	COLOR
        // [0]	COLORV	0.353318453
        // [1]	COLORV	0.353318453
        // [2]	COLORV	0.353318453

        plastic.eval_bsdf(normal, e1, e2, ray, vout);
    }

    #[test]
    fn test_specular_plastic() {
        let plastic = Plastic {
            colour: Spectrum::<{ crate::N_CHANNELS }>([0.2, 0.2, 0.2]),
            specularity: 0.1,
            roughness: 0.1,
        };

        let normal = Vector3D::new(0., 0., 1.);
        let e1 = Vector3D::new(1., 0., 0.);
        let e2 = Vector3D::new(0., 1., 0.);
        let intersection_pt = Point3D::new(0., 0., 0.);

        let mut ray = Ray {
            geometry: Ray3D {
                origin: Point3D::new(-1., 0., 1.),
                direction: Vector3D::new(1., 0., -1.).get_normalized(),
            },
            ..Ray::default()
        };

        let mut rng = crate::rand::get_rng();

        plastic.sample_bsdf(normal, e1, e2, intersection_pt, &mut ray, &mut rng);
    }
}

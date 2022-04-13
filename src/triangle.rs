use crate::Float;
use geometry3d::{BBox3D, Point3D, Ray3D, Vector3D, intersection::{IntersectionInfo, SurfaceSide}};

#[derive(Clone)]
pub struct Triangle {
    pub vertices: [Float;9],
    pub front_material_index: usize,
    pub back_material_index: usize,
}


pub fn world_bounds(t: &Triangle)->BBox3D{
    
    let a = Point3D::new(t.vertices[0], t.vertices[1], t.vertices[2]);
    let bbox = BBox3D::from_point(a);
    
    let b = Point3D::new(t.vertices[3], t.vertices[4], t.vertices[5]);    
    let bbox = BBox3D::from_union_point(&bbox, b);

    let c = Point3D::new(t.vertices[6], t.vertices[7], t.vertices[8]);
    BBox3D::from_union_point(&bbox, c)
}

fn det_3x3(col0: &[Float; 3], col1: &[Float;3], col2: &[Float;3]) -> Float {
    col0[0] * (col1[1] * col2[2]- col2[1] * col1[2]) - 
    col1[0] * (col0[1] * col2[2]- col2[1] * col0[2]) +
    col2[0] * (col0[1] * col1[2]- col1[1] * col0[2])
}
fn baricentric_coorinates(ray: &Ray3D, ax: Float, ay: Float, az: Float, bx: Float, by: Float, bz: Float, cx: Float, cy: Float, cz: Float)-> Option<(Point3D, Float, Float)>{
    
    let ox = ray.origin.x;
    let oy = ray.origin.y;
    let oz = ray.origin.z;

    let dx = ray.direction.x;
    let dy = ray.direction.y;
    let dz = ray.direction.z;


    let a_rox = ax - ox;
    let a_roy = ay - oy;
    let a_roz = az - oz;

    let a_b_x = ax - bx;
    let a_b_y = ay - by;
    let a_b_z = az - bz;
    

    let a_c_x = ax - cx;
    let a_c_y = ay - cy;
    let a_c_z = az - cz;
    

    let a_b = &[a_b_x, a_b_y, a_b_z];
    let a_c = &[a_c_x, a_c_y, a_c_z];
    let rd = &[dx, dy, dz];
    let a_ro = &[a_rox, a_roy, a_roz];
    let det_a = det_3x3(a_b, &a_c, &[dx, dy, dz]);
    

    let u = det_3x3(a_ro, a_c, rd) / det_a;
    let v = det_3x3(a_b, a_ro, rd) / det_a;
    let t = det_3x3(a_b, a_c, a_ro) / det_a;

    // t must be positive, and alpha, beta and gamma must add to 1 and
    // be positive
    if t < 0. || u + v > 1. || u < 0. || v < 0. {
        None
    } else {
        Some((ray.project(t), u, v))
    }

}


pub fn triangle_intersect(t: &Triangle, ray: &geometry3d::Ray3D)->Option<geometry3d::intersection::IntersectionInfo>{
    let ax = t.vertices[0];
    let ay = t.vertices[1];
    let az = t.vertices[2];

    
    let bx = t.vertices[3];    
    let by = t.vertices[4];    
    let bz = t.vertices[5];    

    let cx = t.vertices[6];    
    let cy = t.vertices[7];    
    let cz = t.vertices[8];    

    let (p, _u, _v) = baricentric_coorinates(ray, ax, ay, az, bx, by, bz, cx, cy, cz)?;

    let dpdu = Vector3D::new(ax - bx, ay-by, az-bz);
    let dpdv = Vector3D::new(cx - ax, cy-ay, cz-az);
    // eprintln!("dpdu = {} | dpdv = {}", dpdu, dpdv);
    let normal = dpdu.cross(dpdv).get_normalized();
    // eprintln!("normal = {}", normal);
    let (normal, side) = SurfaceSide::get_side(normal, ray.direction);

    Some(IntersectionInfo {
        p,
        dpdu,
        dpdv,
        normal,
        side,

        #[cfg(feature = "textures")]
        u: _u,
        #[cfg(feature = "textures")]
        v: _v,
        #[cfg(feature = "textures")]
        dndu: Vector3D::new(0., 0., 0.),
        #[cfg(feature = "textures")]
        dndv: Vector3D::new(0., 0., 0.),
    })

}

pub fn simple_triangle_intersect(t: &Triangle, ray: &geometry3d::Ray3D)->Option<geometry3d::Point3D>{
    let ax = t.vertices[0];
    let ay = t.vertices[1];
    let az = t.vertices[2];

    
    let bx = t.vertices[3];    
    let by = t.vertices[4];    
    let bz = t.vertices[5];    

    let cx = t.vertices[6];    
    let cy = t.vertices[7];    
    let cz = t.vertices[8];  
    let (pt, ..) = baricentric_coorinates(ray, ax, ay, az, bx, by, bz, cx, cy, cz)?;
    Some(pt)
}
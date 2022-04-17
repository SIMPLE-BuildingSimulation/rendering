use crate::Float;
use geometry3d::{
    intersection::{IntersectionInfo, SurfaceSide},
    BBox3D, Point3D, Ray3D, Vector3D,
};

pub trait Meshable {
    /// Returns the Triangles and the Normals
    fn mesh()->(Vec<Triangle>, Vec<(Vector3D,Vector3D,Vector3D)>);
}


pub type Triangle = [Float; 9];
    


pub fn world_bounds(t: &Triangle) -> BBox3D {
    let a = Point3D::new(t[0], t[1], t[2]);
    let bbox = BBox3D::from_point(a);

    let b = Point3D::new(t[3], t[4], t[5]);
    let bbox = BBox3D::from_union_point(&bbox, b);

    let c = Point3D::new(t[6], t[7], t[8]);
    BBox3D::from_union_point(&bbox, c)
}


pub fn triangle_pack_baricentric_coorinates(
    ts: &[Triangle],
    ray: &geometry3d::Ray3D,
) -> Option<(usize, geometry3d::Point3D, Float, Float)> {
    let ax = std::simd::Simd::from([
        ts[0][0],
        ts[1][0],
        ts[2][0],
        ts[3][0],
    ]);
    let ay = std::simd::Simd::from([
        ts[0][1],
        ts[1][1],
        ts[2][1],
        ts[3][1],
    ]);
    let az = std::simd::Simd::from([
        ts[0][2],
        ts[1][2],
        ts[2][2],
        ts[3][2],
    ]);

    let bx = std::simd::Simd::from([
        ts[0][3],
        ts[1][3],
        ts[2][3],
        ts[3][3],
    ]);
    let by = std::simd::Simd::from([
        ts[0][4],
        ts[1][4],
        ts[2][4],
        ts[3][4],
    ]);
    let bz = std::simd::Simd::from([
        ts[0][5],
        ts[1][5],
        ts[2][5],
        ts[3][5],
    ]);

    let cx = std::simd::Simd::from([
        ts[0][6],
        ts[1][6],
        ts[2][6],
        ts[3][6],
    ]);
    let cy = std::simd::Simd::from([
        ts[0][7],
        ts[1][7],
        ts[2][7],
        ts[3][7],
    ]);
    let cz = std::simd::Simd::from([
        ts[0][8],
        ts[1][8],
        ts[2][8],
        ts[3][8],
    ]);

    // Calculate baricentric coordinates
    let ox: std::simd::Simd<Float, 4> = std::simd::Simd::splat(ray.origin.x);
    let oy: std::simd::Simd<Float, 4> = std::simd::Simd::splat(ray.origin.y);
    let oz: std::simd::Simd<Float, 4> = std::simd::Simd::splat(ray.origin.z);

    let dx: std::simd::Simd<Float, 4> = std::simd::Simd::splat(ray.direction.x);
    let dy: std::simd::Simd<Float, 4> = std::simd::Simd::splat(ray.direction.y);
    let dz: std::simd::Simd<Float, 4> = std::simd::Simd::splat(ray.direction.z);


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
    let us = u.as_array();
    let vs = v.as_array();
    let ts = t.as_array();
    
    let mut any_intersect = false;
    let mut t = Float::MAX;
    let mut v = Float::MAX;
    let mut u = Float::MAX;
    let mut which_tri = usize::MAX;

    for (i,found_t) in ts.iter().enumerate(){
        let found_u = us[i];
        let found_v = vs[i];
        
        // If it is valid AND is closer than the other 
        let is_valid = *found_t > 0.0 && found_u + found_v < 1. && found_u > 0. && found_v > 0.;
        if is_valid && *found_t < t {             
            any_intersect = true; // mark as found
            t = *found_t;
            u = found_u;
            v = found_v;     
            which_tri = i;       
        }
    }

    if any_intersect {
        Some((which_tri, ray.project(t), u, v))
    }else{
        None
    }
}






fn det_3x3<T>(col0: &[T; 3], col1: &[T; 3], col2: &[T; 3]) -> T 
where T: 
    std::ops::Mul<T, Output=T> + 
    std::ops::Sub<T, Output=T> + 
    std::ops::Add<T, Output=T> + Copy,
{
    col0[0] * (col1[1] * col2[2] - col2[1] * col1[2])
        - col1[0] * (col0[1] * col2[2] - col2[1] * col0[2])
        + col2[0] * (col0[1] * col1[2] - col1[1] * col0[2])
}
fn baricentric_coorinates(
    ray: &Ray3D,
    ax: Float,
    ay: Float,
    az: Float,
    bx: Float,
    by: Float,
    bz: Float,
    cx: Float,
    cy: Float,
    cz: Float,
) -> Option<(Point3D, Float, Float)> {
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



pub fn triangle_intersect(
    t: &Triangle,
    ray: &geometry3d::Ray3D,
) -> Option<geometry3d::intersection::IntersectionInfo> {
    let ax = t[0];
    let ay = t[1];
    let az = t[2];

    let bx = t[3];
    let by = t[4];
    let bz = t[5];

    let cx = t[6];
    let cy = t[7];
    let cz = t[8];

    let (p, _u, _v) = baricentric_coorinates(ray, ax, ay, az, bx, by, bz, cx, cy, cz)?;

    let dpdu = Vector3D::new(ax - bx, ay - by, az - bz);
    let dpdv = Vector3D::new(cx - ax, cy - ay, cz - az);
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

pub fn simple_triangle_intersect(
    t: &Triangle,
    ray: &geometry3d::Ray3D,
) -> Option<geometry3d::Point3D> {
    let ax = t[0];
    let ay = t[1];
    let az = t[2];

    let bx = t[3];
    let by = t[4];
    let bz = t[5];

    let cx = t[6];
    let cy = t[7];
    let cz = t[8];
    let (pt, ..) = baricentric_coorinates(ray, ax, ay, az, bx, by, bz, cx, cy, cz)?;
    Some(pt)
}




pub fn triangle_intersect_pack(
    t: &[Triangle],
    ray: &geometry3d::Ray3D,
) -> Option<(usize, geometry3d::intersection::IntersectionInfo)> {
    
    let (tri_index, p, _u, _v) = triangle_pack_baricentric_coorinates(t, ray)?;

    let ax = t[tri_index][0];
    let ay = t[tri_index][1];
    let az = t[tri_index][2];

    let bx = t[tri_index][3];
    let by = t[tri_index][4];
    let bz = t[tri_index][5];

    let cx = t[tri_index][6];
    let cy = t[tri_index][7];
    let cz = t[tri_index][8];

    let dpdu = Vector3D::new(ax - bx, ay - by, az - bz);
    let dpdv = Vector3D::new(cx - ax, cy - ay, cz - az);
    // eprintln!("dpdu = {} | dpdv = {}", dpdu, dpdv);
    let normal = dpdu.cross(dpdv).get_normalized();
    // eprintln!("normal = {}", normal);
    let (normal, side) = SurfaceSide::get_side(normal, ray.direction);

    Some((tri_index,IntersectionInfo {
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
    }))
}

pub fn simple_triangle_intersect_pack(
    t: &[Triangle],
    ray: &geometry3d::Ray3D,
) -> Option<(usize, geometry3d::Point3D)> {    
    let (tri_index, pt, ..) = triangle_pack_baricentric_coorinates(t, ray)?;
    Some((tri_index, pt))
}
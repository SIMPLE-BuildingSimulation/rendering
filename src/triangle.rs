use crate::Float;
use geometry3d::{
    intersection::{IntersectionInfo, SurfaceSide},
    BBox3D, Point3D, Ray3D, Vector3D, Triangle3D, Sphere3D,
};





/// The smallest definition of a Triangle I could think of
pub type Triangle = [Float; 9];

pub fn triangle_area(triangle: &Triangle)->Float{

    let a = std::simd::Simd::from_array([ triangle[0], triangle[1], triangle[2], 0.0 ]);
    let b = std::simd::Simd::from_array([ triangle[3], triangle[4], triangle[5], 0.0 ]);
    let c = std::simd::Simd::from_array([ triangle[6], triangle[7], triangle[8], 0.0 ]);

    let mut ab = b - a;
    ab *= ab;
    let ab = ab.reduce_sum().sqrt();

    let mut bc = c - b;
    bc *= bc;
    let bc = bc.reduce_sum().sqrt();

    let mut ca = c - a;
    ca *= ca;
    let ca = ca.reduce_sum().sqrt();
    

    ((ca + bc + ab)
        * ((ca + bc + ab) / 2. - ab)
        * ((ca + bc + ab) / 2. - bc)
        * ((ca + bc + ab) / 2. - ca)
        / 2.)
        .sqrt()
}

pub fn triangle_solid_angle_pdf(
    triangle: &Triangle,
    point: Point3D,
    normal: Vector3D,
    ray: &Ray3D,
) -> Float {
    let d2 = (point - ray.origin).length_squared();
    let cos_theta = ray.origin * normal;
    // debug_assert!(cos_theta > 0.);
    if cos_theta < 1e-7 {
        return 0.0;
    }
    let area = triangle_area(triangle);
    // return
    d2 / cos_theta.abs() / area
}



/// Gets the BBox of a Triangle
pub fn world_bounds(t: &Triangle) -> BBox3D {
    let a = Point3D::new(t[0], t[1], t[2]);
    let bbox = BBox3D::from_point(a);

    let b = Point3D::new(t[3], t[4], t[5]);
    let bbox = BBox3D::from_union_point(&bbox, b);

    let c = Point3D::new(t[6], t[7], t[8]);
    BBox3D::from_union_point(&bbox, c)
}

/// Tests the intersection between a `Ray3D` and a pack (i.e., `&[]`) 
/// of [`Triangle`]. Returns the index of the intersected triangle within the
/// pack, the point of intersection, and the `u` and `v` baricentric coordinates 
/// of the intersection point.
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





/// Calculates the determinant of a 3x3 matrix
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


/// Tests the intersection between a `Ray3D` and a 
/// [`Triangle`]. Returns the the point of intersection, and the `u` 
/// and `v` baricentric coordinates of the intersection point.
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


/// Intersects a `Ray3D` and a [`Triangle`], returning the [`IntersectionInfo`]
/// (or `None` if they don't intersect)
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

    let (p, u, v) = baricentric_coorinates(ray, ax, ay, az, bx, by, bz, cx, cy, cz)?;

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

        u,
        v,
        dndu: Vector3D::new(0., 0., 0.),        
        dndv: Vector3D::new(0., 0., 0.),
    })
}


/// Intersects a `Ray3D` and a [`Triangle`], returning the `Point3D` of 
/// intersection
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



/// Intersects a `Ray3D` and a pack (i.e., `&[]`) of [`Triangle`], returning the
/// index of the intersected [`Triangle`] within the pack, and its [`IntersectionInfo`]
/// (or `None` if they don't intersect)
pub fn triangle_intersect_pack(
    t: &[Triangle],
    ray: &geometry3d::Ray3D,
) -> Option<(usize, geometry3d::intersection::IntersectionInfo)> {
    
    let (tri_index, p, u, v) = triangle_pack_baricentric_coorinates(t, ray)?;

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
        u,
        v,        
        dndu: Vector3D::new(0., 0., 0.),        
        dndv: Vector3D::new(0., 0., 0.),
    }))
}


/// Intersects a `Ray3D` and a pack (i.e., `&[]`) of [`Triangle`], returning the
/// index of the intersected [`Triangle`] within the pack, and the `Point3D` of 
/// intersection
pub fn simple_triangle_intersect_pack(
    t: &[Triangle],
    ray: &geometry3d::Ray3D,
) -> Option<(usize, geometry3d::Point3D)> {    
    let (tri_index, pt, ..) = triangle_pack_baricentric_coorinates(t, ray)?;
    Some((tri_index, pt))
}


/// Transforms a `Triangle3D` and transforms it into a `Vec<Triangle>` and their 
/// respective normals
pub fn mesh_triangle(tr: &Triangle3D)->(Vec<Triangle>, Vec<(Vector3D,Vector3D,Vector3D)>){
    // Become a single triangle... dah!
    let s1 = tr.b() - tr.a();
    let s2 = tr.c() - tr.a();

    
    // All vertices have the same normal
    let normal = s1.cross(s2).get_normalized();
    let normals = vec![(normal, normal, normal)];                
        
    // Push triangle
    let triangles = vec![[
        tr.a().x, tr.a().y, tr.a().z, 
        tr.b().x, tr.b().y, tr.b().z,
        tr.c().x, tr.c().y, tr.c().z
    ]];
    (triangles,normals)
}


pub fn mesh_sphere(s: &Sphere3D)->(Vec<Triangle>, Vec<(Vector3D,Vector3D,Vector3D)>){    
    const N_REFINEMENTS: u32 = 5;

    let r = s.radius;
    let c = s.centre();
    // check if partial
    let bounds = s.bounds();
    let zmin = bounds.min.z;
    let zmax = bounds.max.z;
    if zmin > -r || zmax < r {
        eprintln!("Warning: Partial Sphere Meshing is not supported yet... adding it as a full sphere.")
    }
    
    // Initialize: set basic coordinates
    let midtop = r*(60. as Float).to_radians().cos();
    let midr = r*(60. as Float).to_radians().sin();    
    let midbottom = -midtop;
    // Points
    let top = Point3D::new(0., 0., r) + c;
    let bottom = Point3D::new(0., 0., -r) + c;
    let midtop : Vec<Point3D> = vec![ 36., 3.*36., 5.*36., 7.*36., 9.*36. ].iter().map(|angle : &Float| Point3D::new(midr*angle.to_radians().sin(), midr*angle.to_radians().cos(), midtop) + c ).collect();
    let midbottom : Vec<Point3D> = vec![ 0., 72., 2.*72., 3.*72., 4.*72. ].iter().map(|angle : &Float| Point3D::new(midr*angle.to_radians().sin(), midr*angle.to_radians().cos(), midbottom) + c ).collect();
    
    let mut triangles : Vec<(Point3D, Point3D, Point3D)> = Vec::with_capacity((4 as usize).pow(N_REFINEMENTS) * 20);
    
    // In reverse (to respect the triangle's normal direction)
    triangles.push((midtop[0], midtop[4], top));
    triangles.push((midtop[4], midtop[3], top));
    triangles.push((midtop[3], midtop[2], top));
    triangles.push((midtop[2], midtop[1], top));
    triangles.push((midtop[1], midtop[0], top));

    triangles.push((midbottom[0], midbottom[1], bottom));
    triangles.push((midbottom[1], midbottom[2], bottom));
    triangles.push((midbottom[2], midbottom[3], bottom));
    triangles.push((midbottom[3], midbottom[4], bottom));
    triangles.push((midbottom[4], midbottom[0], bottom));

    triangles.push((midtop[4], midtop[0], midbottom[0]));
    triangles.push((midtop[0], midtop[1], midbottom[1]));
    triangles.push((midtop[1], midtop[2], midbottom[2]));
    triangles.push((midtop[2], midtop[3], midbottom[3]));
    triangles.push((midtop[3], midtop[4], midbottom[4]));

    triangles.push((midbottom[1], midbottom[0], midtop[0]));
    triangles.push((midbottom[2], midbottom[1], midtop[1]));
    triangles.push((midbottom[3], midbottom[2], midtop[2]));
    triangles.push((midbottom[4], midbottom[3], midtop[3]));
    triangles.push((midbottom[0], midbottom[4], midtop[4]));
 
    // Refine
    let centre = s.centre();
    let mut refine  = ||{
        let n = triangles.len();
        for i in 0..n{            
            let (a, b, c) = triangles[i];
            // interpolate
            let ab = (a + b)/2.;            
            let ac = (a + c)/2.;
            let bc = (b + c)/2.;
            // project into the sphere
            let ab = centre + (ab - centre).get_normalized()*r;
            let ac = centre + (ac - centre).get_normalized()*r;
            let bc = centre + (bc - centre).get_normalized()*r;


            // Replace existing one
            triangles[i] = (a, ab, ac);

            // push others at the back
            triangles.push( (ab, b, bc) );
            triangles.push( (bc, c, ac) );
            triangles.push( (ab, bc, ac) );
        }
    };
    
    for _ in 0..N_REFINEMENTS{
        refine()
    }
    

    // Transform    
    let normals : Vec<(Vector3D, Vector3D, Vector3D)> = triangles.iter().map(|vertex|{
        let n0 = (vertex.0 - centre).get_normalized();
        let n1 = (vertex.1 - centre).get_normalized();
        let n2 = (vertex.2 - centre).get_normalized();
        (n0, n1, n2)
    }).collect();
    let triangles : Vec<Triangle> = triangles.iter().map(|vertex|{
        [vertex.0.x, vertex.0.y, vertex.0.z, vertex.1.x, vertex.1.y, vertex.1.z, vertex.2.x, vertex.2.y, vertex.2.z]
    }).collect();
    (triangles, normals)

}
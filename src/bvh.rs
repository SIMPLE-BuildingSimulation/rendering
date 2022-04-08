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

/*
THIS CODE IS HEAVILY INSPIRED (rip-off?) IN
Physically Based Rendering: From Theory To Implementation, © 2004-2021 Matt Pharr, Wenzel Jakob, and Greg Humphreys
https://pbr-book.org/3ed-2018/Primitives_and_Intersection_Acceleration/Bounding_Volume_Hierarchies
 */

use crate::ray::Ray;
use crate::scene::{Object, Scene};
use crate::Float;
use geometry3d::{BBox3D, BBoxAxis, Point3D, Ray3D, Vector3D};
use std::cmp::Ordering;

#[derive(Copy, Clone)]
struct BucketInfo {
    count: usize,
    bounds: Option<BBox3D>,
}

fn get_bucket_index(
    centroid: Point3D,
    len_axis: Float,
    split_axis: BBoxAxis,
    n_buckets: usize,
    min_centroid: Point3D,
) -> usize {
    // Identify which bucket contains this object's centroid
    let (centroid_pos, min) = match split_axis {
        BBoxAxis::X => (centroid.x, min_centroid.x),
        BBoxAxis::Y => (centroid.y, min_centroid.y),
        BBoxAxis::Z => (centroid.z, min_centroid.z),
    };
    let mut bucket_index = ((centroid_pos - min) * n_buckets as Float / len_axis).floor() as usize;
    debug_assert!(bucket_index <= n_buckets);
    if bucket_index == n_buckets {
        // If we are in the upper limit, this can happen
        bucket_index = n_buckets - 1;
    }

    bucket_index
}

/// A struct that is instrumental for building the  `BoundingVolumeTree`
struct ObjectInfo {
    index: usize,
    bounds: BBox3D,
    centroid: Point3D,
}

impl ObjectInfo {
    fn new(index: usize, object: &Object) -> Self {
        let bounds = object.primitive.world_bounds();
        let centroid = (bounds.max + bounds.min) * 0.5;
        Self {
            index,
            bounds,
            centroid,
        }
    }
}

#[derive(Clone)]
struct Leaf {
    bounds: BBox3D,
    n_prims: usize,
    first_prim_offset: usize,
}

struct Interior {
    bounds: BBox3D,
    split_axis: BBoxAxis,
    children: (Box<Node>, Box<Node>),
}

enum Node {
    Interior(Interior),
    Leaf(Leaf),
}

impl Node {
    fn bounds(&self) -> BBox3D {
        match self {
            Self::Leaf(l) => l.bounds,
            Self::Interior(i) => i.bounds,
        }
    }

    fn new_leaf(first: usize, n_prims: usize, bounds: BBox3D) -> Self {
        Self::Leaf(Leaf {
            bounds,
            first_prim_offset: first,
            n_prims,
        })
    }

    fn new_interior(split_axis: BBoxAxis, child1: Node, child2: Node) -> Self {
        let bounds = BBox3D::from_union(&child1.bounds(), &child2.bounds());

        Self::Interior(Interior {
            bounds,
            split_axis,
            children: (Box::new(child1), Box::new(child2)),
        })
    }

    fn recursive_build(
        primitives: &[Object],
        primitives_info: &mut [ObjectInfo],
        start: usize,
        end: usize,
        total_nodes: &mut usize,
        ordered_primes: &mut Vec<Object>,
    ) -> Self {
        debug_assert!(start < end);
        *total_nodes += 1;
        // The whole point of most of this function is to idenfity
        // the value this 'mid' variable should have... we are creating
        // a binary tree, you know, so we split things in halves
        let mut mid: Option<usize> = None;

        // Get a BBOX containing EVERYTHING within scope
        let mut bounds = primitives_info[start].bounds;
        // for i in start+1..end{
        //     bounds = BBox3D::from_union(&bounds, &primitives_info[i].bounds);
        // }
        for info in primitives_info.iter().take(end).skip(start + 1) {
            bounds = BBox3D::from_union(&bounds, &info.bounds);
        }
        let n_primitives = end - start;
        if n_primitives == 1 {
            // Create Leaf
            let first_prim_offset = ordered_primes.len();
            for info in primitives_info.iter().take(end).skip(start) {
                let index = info.index;
                ordered_primes.push(primitives[index].clone())
            }
            return Node::new_leaf(first_prim_offset, n_primitives, bounds);
        }

        // Calculate the the BBOX of the centroids
        let mut centroids_bbox = BBox3D::from_point(primitives_info[start].centroid);
        for prim_info in primitives_info.iter().take(end).skip(start + 1) {
            centroids_bbox = BBox3D::from_union_point(&centroids_bbox, prim_info.centroid);
        }

        let split_axis = centroids_bbox.max_extent();
        // the extent of the centroids in the largest dimension
        let len_axis = match split_axis {
            BBoxAxis::X => centroids_bbox.max.x - centroids_bbox.min.x,
            BBoxAxis::Y => centroids_bbox.max.y - centroids_bbox.min.y,
            BBoxAxis::Z => centroids_bbox.max.z - centroids_bbox.min.z,
        };
        // Define some auxiliary functions
        let cmp_float = |a: Float, b: Float| -> Ordering {
            if a < b {
                Ordering::Less
            } else if a > b {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        };
        let cmp_centroids = |a: &ObjectInfo, b: &ObjectInfo| -> Ordering {
            match split_axis {
                BBoxAxis::X => cmp_float(a.centroid.x, b.centroid.x),
                BBoxAxis::Y => cmp_float(a.centroid.y, b.centroid.y),
                BBoxAxis::Z => cmp_float(a.centroid.z, b.centroid.z),
            }
        };

        const TOO_FEW_TO_BUCKET: usize = 2;
        const N_BUCKETS: usize = 12;
        const RELATIVE_TRANSVERSAL_COST: Float = 2.;
        // Now, Proceed
        if len_axis < 1e-8 {
            // All primitives seem to be aligned in all directions (i.e., overlapping)
            // Put them al together in a Leaf
            let first_prim_offset = ordered_primes.len();
            // for i in start..end{
            for prim_info in primitives_info.iter().take(end).skip(start) {
                let index = prim_info.index;
                ordered_primes.push(primitives[index].clone())
            }
            return Node::new_leaf(first_prim_offset, n_primitives, bounds);
        } else {
            // Line 306 of https://github.com/mmp/pbrt-v3/blob/master/src/accelerators/bvh.cpp
            if n_primitives <= TOO_FEW_TO_BUCKET {
                // too few... just split in half,
                let this_mid = (end - start) / 2;

                mid = Some(this_mid); // this operation rounds results
                                      // and sort the relevant range in the array
                primitives_info[start..end].select_nth_unstable_by(this_mid, cmp_centroids);
            } else {
                // Use the Surface Area Heuristic...

                // First, put all the elements in a bucket
                let mut buckets: Vec<BucketInfo> = vec![
                    BucketInfo {
                        count: 0,
                        bounds: None
                    };
                    N_BUCKETS
                ];

                for prim_info in primitives_info.iter().take(end).skip(start) {
                    // Identify which bucket contains this object's centroid
                    let bucket_index = get_bucket_index(
                        prim_info.centroid,
                        len_axis,
                        split_axis,
                        N_BUCKETS,
                        centroids_bbox.min,
                    );

                    // Register
                    buckets[bucket_index].count += 1;
                    match buckets[bucket_index].bounds {
                        Some(b) => {
                            buckets[bucket_index].bounds =
                                Some(BBox3D::from_union_point(&b, prim_info.centroid))
                        }
                        None => {
                            buckets[bucket_index].bounds =
                                Some(BBox3D::from_point(prim_info.centroid))
                        }
                    }
                }

                // Compute costs of splitting after each bucket
                let mut min_cost = Float::MAX;
                let mut min_cost_bucket = 0;
                for i in 0..N_BUCKETS - 1 {
                    // Properties before... initialize as first bucket
                    let mut before = buckets[0].bounds.unwrap();
                    let mut count_before = buckets[0].count;

                    for bucket in buckets.iter().take(i + 1).skip(1) {
                        if let Some(b) = &bucket.bounds {
                            before = BBox3D::from_union(&before, b);
                            count_before += bucket.count;
                        }
                    }
                    // Properties after... initialize as last bucket
                    let mut after = buckets[N_BUCKETS - 1].bounds.unwrap();
                    let mut count_after = buckets[N_BUCKETS - 1].count;
                    // for j in (i+1)..(N_BUCKETS-1){
                    for bucket in buckets.iter().take(N_BUCKETS - 1).skip(i + 1) {
                        if let Some(b) = &bucket.bounds {
                            after = BBox3D::from_union(&after, b);
                            count_after += bucket.count;
                        }
                    }

                    let c = RELATIVE_TRANSVERSAL_COST
                        + (count_before as Float * before.surface_area()
                            + count_after as Float * after.surface_area())
                            / bounds.surface_area();
                    if c < min_cost {
                        min_cost = c;
                        min_cost_bucket = i;
                    }
                } // end of calculating bucket's costs

                // Decide whether to keep splitting or not.
                let leaf_cost = n_primitives as Float;
                if n_primitives > 24 || min_cost < leaf_cost {
                    // We need or want to subdivide... create interior

                    // Sort based on centroid position
                    primitives_info[start..end].sort_unstable_by(cmp_centroids);
                    // Identify the first primitive that is
                    for (index, i) in (start..end).into_iter().enumerate() {
                        let bucket_index = get_bucket_index(
                            primitives_info[i].centroid,
                            len_axis,
                            split_axis,
                            N_BUCKETS,
                            centroids_bbox.min,
                        );
                        if bucket_index <= min_cost_bucket {
                            mid = Some(index + 1) // update mid.
                        } else {
                            break; // we are past (these are sorted)... just break and prepare to return.
                        }
                    }
                } else {
                    // Don't subdivide... create leaf
                    let first = ordered_primes.len();
                    let n_prims = n_primitives;
                    for prim in primitives_info.iter().take(end).skip(start) {
                        let prim_num = prim.index;
                        ordered_primes.push(primitives[prim_num].clone());
                    }
                    return Node::new_leaf(first, n_prims, bounds);
                }
            }
        }
        // If we have not returned a Leaf yet... split!
        let mid = mid.unwrap() + start;
        let child1 = Self::recursive_build(
            primitives,
            primitives_info,
            start,
            mid,
            total_nodes,
            ordered_primes,
        );
        let child2 = Self::recursive_build(
            primitives,
            primitives_info,
            mid,
            end,
            total_nodes,
            ordered_primes,
        );
        Node::new_interior(split_axis, child1, child2)
    }
}

type FlatLeaf = Leaf;
#[derive(Clone)]
struct FlatInterior {
    second_child_offset: usize,
    bounds: BBox3D,
    axis: BBoxAxis,
}
#[derive(Clone)]
enum FlatNode {
    Interior(FlatInterior),
    Leaf(FlatLeaf),
}

impl FlatNode {
    fn bounds(&self) -> BBox3D {
        match self {
            Self::Interior(i) => i.bounds,
            Self::Leaf(i) => i.bounds,
        }
    }
}

#[derive(Default, Clone)]
pub struct BoundingVolumeTree{
    nodes: Vec<FlatNode>,
}

impl BoundingVolumeTree {
    pub fn new(scene: &mut Scene) -> Self {
        let n_objects = scene.objects.len();
        if n_objects == 0 {
            return Self::default();
        }
        /*
        STEP 1:  First, bounding information about each primitive is computed and
        stored in an array that will be used during tree construction
        */
        let mut primitives_info: Vec<ObjectInfo> = Vec::with_capacity(n_objects);

        for (i, ob) in scene.objects.iter().enumerate() {
            primitives_info.push(ObjectInfo::new(i, ob))
        }

        /*
        STEP 2:  Next, the tree is built using the algorithm choice
        encoded in splitMethod. The result is a binary tree where
        each interior node holds pointers to its children and each
        leaf node holds references to one or more primitives.
        */
        let mut total_nodes = 0;
        let mut ordered_primitives: Vec<Object> = Vec::with_capacity(n_objects);
        let root = Node::recursive_build(
            &scene.objects,
            &mut primitives_info,
            0,
            n_objects,
            &mut total_nodes,
            &mut ordered_primitives,
        );
        scene.objects = ordered_primitives; // Update the Scene with the ordered primitive.

        /*
        STEP 3: Finally, this tree is converted to a more compact
        (and thus more efficient) pointerless representation for
        use during rendering.
        */
        let mut nodes: Vec<FlatNode> = Vec::with_capacity(total_nodes);
        Self::flatten_node(&root, &mut nodes);

        // return
        Self{
            nodes            
        }
    }

    fn flatten_node(node: &Node, nodes: &mut Vec<FlatNode>) -> usize {
        let this_offset = nodes.len();
        match node {
            Node::Leaf(l) => {
                nodes.push(FlatNode::Leaf(FlatLeaf {
                    bounds: l.bounds,
                    n_prims: l.n_prims,
                    first_prim_offset: l.first_prim_offset,
                }));
            }
            Node::Interior(i) => {
                let (child1, child2) = &i.children;
                nodes.push(FlatNode::Interior(FlatInterior {
                    second_child_offset: 0, // We will fill this in a minute.
                    bounds: i.bounds,
                    axis: i.split_axis,
                }));
                Self::flatten_node(child1, nodes);
                let second_offset = Self::flatten_node(child2, nodes);
                // Patch second offset
                if let FlatNode::Interior(this_node) = &mut nodes[this_offset] {
                    this_node.second_child_offset = second_offset;
                } else {
                    unreachable!()
                }
            }
        }
        // return
        this_offset
    }

    /// Returns an `Option<Interaction>`, containing the first primitive
    /// to be hit by the ray, if any
    pub fn intersect(&self, primitives: &[Object], ray: &mut Ray, nodes_to_visit: &mut Vec<usize>) -> bool {
        const MIN_T: Float = 0.0000001;

        if self.nodes.is_empty() {
            return false;
        }
        // reset
        nodes_to_visit.truncate(0);
        
        let mut prim_index: Option<usize> = None;
        let mut t_squared = Float::MAX;

        let inv_dir = Vector3D::new(
            1. / ray.geometry.direction.x,
            1. / ray.geometry.direction.y,
            1. / ray.geometry.direction.z,
        );
        let dir_is_neg = (inv_dir.x < 0., inv_dir.y < 0., inv_dir.z < 0.);
        let mut current_node = 0;
        
        loop {
            let node = &self.nodes[current_node];
            if node.bounds().intersect(&ray.geometry, inv_dir) {
                match node {
                    FlatNode::Leaf(data) => {
                        let offset = data.first_prim_offset;
                        // Check all the objects in this Node
                        for i in 0..data.n_prims {
                            if let Some(intersect_info) =
                                primitives[offset + i].primitive.intersect(&ray.geometry)
                            {
                                // If hit, check the distance.
                                let this_t_squared =
                                    (intersect_info.p - ray.geometry.origin).length_squared();
                                // if the distance is less than the prevous one, update the info
                                if this_t_squared > MIN_T && this_t_squared < t_squared {
                                    // If the distance is less than what we had, update return data
                                    t_squared = this_t_squared;
                                    prim_index = Some(offset + i);
                                    ray.interaction.geometry_shading = intersect_info;
                                }
                            }
                        }
                        if let Some(i) = nodes_to_visit.pop() {
                            current_node = i;
                        } else {
                            break;
                        }
                    }
                    FlatNode::Interior(data) => {
                        let is_neg = match data.axis {
                            BBoxAxis::X => dir_is_neg.0,
                            BBoxAxis::Y => dir_is_neg.1,
                            BBoxAxis::Z => dir_is_neg.2,
                        };
                        if is_neg {
                            nodes_to_visit.push(current_node + 1);
                            current_node = data.second_child_offset;
                        } else {
                            nodes_to_visit.push(data.second_child_offset);
                            current_node += 1;
                        }
                    }
                }
            } else if let Some(i) = nodes_to_visit.pop() {
                current_node = i;
            } else {
                break;
            }
        } // End loop

        // return
        if let Some(index) = prim_index {
            
            let t = t_squared.sqrt();
            
            ray.interaction.point = ray.geometry.project(t);
            ray.interaction.wo = ray.geometry.direction * -1.;
            ray.interaction.prim_index = index;

            true
        } else {
            false
        }
    }

    /// Checks if a ray can travel a certain distance without hitting anything
    pub fn unobstructed_distance(
        &self,
        primitives: &[Object],
        ray: &Ray3D,
        distance_squared: Float,
        nodes_to_visit: &mut Vec<usize>,
    ) -> bool {
        if self.nodes.is_empty() {
            return true;
        }
        // reset
        nodes_to_visit.truncate(0);
        // let d_squared = distance * distance;
        const MIN_T: Float = 0.000001;

        let inv_dir = Vector3D::new(
            1. / ray.direction.x,
            1. / ray.direction.y,
            1. / ray.direction.z,
        );
        let dir_is_neg = (inv_dir.x < 0., inv_dir.y < 0., inv_dir.z < 0.);
        let mut current_node = 0;
        
        loop {
            let node = &self.nodes[current_node];
            if node.bounds().intersect(ray, inv_dir) {
                match node {
                    FlatNode::Leaf(data) => {
                        let offset = data.first_prim_offset;
                        // Check all the objects in this Node
                        for i in 0..data.n_prims {
                            if let Some(pt) = primitives[offset + i].primitive.simple_intersect(ray)
                            {
                                let this_d_squared = (pt - ray.origin).length_squared();

                                // Is it a valid hit and it is earlier than the rest?
                                if this_d_squared > MIN_T
                                    && this_d_squared + MIN_T < distance_squared
                                    && (distance_squared - this_d_squared).abs() > 0.0001
                                {
                                    return false;
                                }
                            }
                        }
                        if let Some(i) = nodes_to_visit.pop() {
                            current_node = i;
                        } else {
                            break;
                        }
                    }
                    FlatNode::Interior(data) => {
                        let is_neg = match data.axis {
                            BBoxAxis::X => dir_is_neg.0,
                            BBoxAxis::Y => dir_is_neg.1,
                            BBoxAxis::Z => dir_is_neg.2,
                        };
                        if is_neg {
                            nodes_to_visit.push(current_node + 1);
                            current_node = data.second_child_offset;
                        } else {
                            nodes_to_visit.push(data.second_child_offset);
                            current_node += 1;
                        }
                    }
                }
            } else if let Some(i) = nodes_to_visit.pop() {
                current_node = i;
            } else {
                break;
            }
        } // End loop

        // otherwise, return true
        true
    }
}

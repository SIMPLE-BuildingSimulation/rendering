use crate::scene::Scene;
use crate::vertex::Vertex;

const MAX_VERTICES: usize = 12;

#[derive(Clone, Copy)]
pub struct SubPath {
    vertices: [Option<Vertex>; MAX_VERTICES],
    pub n_vertices: usize,
}

impl std::ops::Index<usize> for SubPath {
    type Output = Vertex;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= MAX_VERTICES {
            panic!(
                "Trying to acces Vertex out of bounds in SubPath: index is {} and len() is {}",
                index,
                self.vertices.len()
            );
        }
        match &self.vertices[index] {
            Some(v) => v,
            None => panic!(
                "Trying to access a None vertex in Subpath... index is {}",
                index
            ),
        }
    }
}

impl SubPath {
    /// Creates a new empty path
    pub fn new() -> Self {
        Self {
            vertices: [None; MAX_VERTICES],
            n_vertices: 0,
        }
    }

    pub fn random_walk(&mut self, scene: &Scene, max_depth: usize, rroulet: f64) {
        unimplemented!();
    }

    /// Pushes a new [`Vertex`]; returns the index of that [`Vertex`]
    pub fn push(&mut self, v: Vertex) -> usize {
        // This needs to be None, for now.
        debug_assert!(self.vertices[self.n_vertices].is_none());
        // Now we initialize it
        self.vertices[self.n_vertices] = Some(v);
        self.n_vertices += 1;
        self.n_vertices - 1
    }

    // Extends both sides of the path
    // fn alternate_walk(light_subpath:&mut Self, max_source_subpath_steps: usize, eye_subpath:&mut Self, max_eye_subpath_steps: usize, rroulete: f64){
    //     let this_source_ver = light_subpath.last();
    //     let this_eye_vert

    //     // mark the first two points from eye

    //     // Random walk

    // }

    // Evaluates the path combinations
    // fn evaluate(light_subpath:&Self, eye_subpath:&Self)->Spectrum{

    // }
}

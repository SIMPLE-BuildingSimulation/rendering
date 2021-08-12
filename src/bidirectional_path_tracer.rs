use crate::Float;
use crate::camera::{Camera, CameraSample};
use crate::colour::Spectrum;
use crate::image::ImageBuffer;
use crate::scene::Scene;
use crate::subpath::SubPath;
use crate::vertex::{Vertex, VertexData};

fn evaluate_path(
    samples: &mut ImageBuffer,
    light_subpath: &SubPath,
    eye_subpath: &SubPath,
) -> Spectrum {
    unimplemented!();
}

/// Generates the Light [`Subpath`]
///
/// From Veach's thesis, p. 298: "the light subpath... is constructed by choosing a random point... on
/// a light source, followed by casting a ray in a random direction"
fn get_light_subpath(light_i: usize, scene: &Scene, max_depth: usize, rroulet: Float) -> SubPath {
    let object_index = scene.light(light_i);
    let light = scene.borrow_object(object_index);
    // There must be a more efficient way of doing this..?
    let light_sampler = light.primitive.surface_sampler(1);
    let light_p = light_sampler.next().unwrap();

    // We are assuming that the light emits uniformly
    let mut ret = SubPath::new();    
    ret.push(Vertex::Camera(VertexData {
        normal: None,
        position: light_p,
        material_index: Some(light.front_material_index),
        object_index: None,
        is_specular: false,
    }));

}

/// Generates the Eye [`Subpath`]
///
/// From Veach's thesis, p. 298: "The eye subpath ... is constructed by a similar process starting
/// from a random point on the camera lens."
fn get_eye_subpath(scene: &Scene, camera: &dyn Camera, max_depth: usize, rroulet: Float) -> SubPath {
    
    // Get a camera vertex
    let (x_pos, y_pos) = rand::random::<(Float, Float)>();
    let (f_width, f_height) = camera.film_resolution();
    let x = (x_pos * f_width as Float).round() as usize;
    let y = (y_pos * f_height as Float).round() as usize;
    debug_assert!(x <= f_width);
    debug_assert!(y <= f_height);
    let sample = CameraSample {
        p_film: (x, y),
        p_lens: (0., 0.), // we still do not use this
        time: 0.,         // we still do not use this
    };
    let (ray, _weight) = camera.gen_ray(&sample);

    let mut ret = SubPath::new();
    let view = camera.view();
    let beta = Spectrum::black();// this is probably wrong
    ret.push(Vertex::new_in_camera(ray,view.view_direction, beta));

    // Random walk
    ret.random_walk(scene, max_depth - 1, rroulet);
    // return
    ret
}

pub struct BidPathTracer {}

impl BidPathTracer {
    /// Process a single camera ray
    pub fn render(scene: &Scene, camera: &dyn Camera) -> ImageBuffer {
        const MAX_SOURCE_SUBPATH_DEPTH: usize = 6;
        const MAX_EYE_SUBPATH_DEPTH: usize = 6;
        const RROULETE: Float = 0.1;

        let (width, height) = camera.film_resolution();

        let mut samples = ImageBuffer::new(width, height);
        if scene.n_lights() == 0 {
            return samples;
        }

        for y in 0..height {
            for x in 0..width {
                for light_i in 0..scene.n_lights() {
                    /*
                    Veach's thesis, p. 298

                    Each technique samples a path by connecting two independently
                    generated pieces, one starting from the light sources, and the
                    other from the eye.
                    */
                    let light_subpath =
                        get_light_subpath(light_i, scene, MAX_SOURCE_SUBPATH_DEPTH, RROULETE);
                    let eye_subpath =
                        get_eye_subpath(scene, camera, MAX_EYE_SUBPATH_DEPTH, RROULETE);
                    evaluate_path(&mut samples, &light_subpath, &eye_subpath);
                } // end of n_lights
            } // end of x
        } // end of y

        samples
    }
}

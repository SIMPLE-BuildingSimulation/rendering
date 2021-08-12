// Define whether we are working with
// doubles (i.e., f64) or floats (i.e., f32)

/// The kind of Floating point number used in the
/// library... the `"float"` feature means it becomes `f32`
/// and `f64` is used otherwise.
#[cfg(feature = "float")]
type Float = f32;
#[cfg(feature = "float")]
const PI: Float = std::f32::consts::PI;

#[cfg(not(feature = "float"))]
type Float = f64;
#[cfg(not(feature = "float"))]
const PI: Float = std::f64::consts::PI;


// Core
pub mod ray;
pub mod interaction;
pub mod sampleable_trait;
pub mod samplers;
pub mod camera;
pub mod colour;
pub mod film;
pub mod from_radiance;
pub mod image;
pub mod material;
pub mod scene;
// pub mod lights;

// Ray-tracer
pub mod ray_tracer;

// Bidirectional Path Tracing modules
// pub mod bidirectional_path_tracer;
// pub mod subpath;
// pub mod vertex;

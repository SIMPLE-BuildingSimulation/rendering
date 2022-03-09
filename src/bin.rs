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

use rendering::scene::Scene;
// use rendering::from_radiance::from
use clap::{Command, Arg};

use geometry3d::{Point3D, Vector3D};
use rendering::camera::{Camera, View};
use rendering::film::Film;
use rendering::ray_tracer::RayTracer;
use std::time::Instant;

fn main() {
    let matches = Command::new("SIMPLE ray tracer")
        .version("0.1 (but it is still awesome!)")
        .author("(c) German Molina")
        .about("A simple ray-tracing renderer")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("Radiance file")
                .help("This is the SIMPLE Model or a Radiance file")
                .takes_value(true)
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("The file where to write the image")
                .help("The file where to write the image")
                .takes_value(true)
                .required(true)
                .index(2),
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();
    let mut scene = Scene::from_radiance(input_file.to_string());

    scene.build_accelerator();

    // Create camera
    let film = Film {
        // resolution: (512, 367),
        resolution: (1024, 768),
        // resolution: (512, 512),
    };

    // Create view
    let view = View {
        view_direction: Vector3D::new(0., 1., 0.).get_normalized(),
        // view_point: Point3D::new(2., 1., 1.),
        view_point: Point3D::new(3., -5., 2.25),
        field_of_view: 50.,
        ..View::default()
    };

    // Create camera
    let camera = Camera::pinhole(view, film);

    let integrator = RayTracer {
        n_shadow_samples: 10,
        max_depth: 3,
        limit_weight: 0.001,
        n_ambient_samples: 1890,
        ..RayTracer::default()
    };

    let now = Instant::now();

    let buffer = integrator.render(&scene, &camera);
    println!(
        "Image described in '{}' took {} seconds to render",
        input_file,
        now.elapsed().as_secs()
    );
    buffer.save_hdre(output_file.to_string());
}
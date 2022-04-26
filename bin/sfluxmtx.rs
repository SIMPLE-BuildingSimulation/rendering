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
use solar::ReinhartSky;
// use rendering::from_radiance::from
use clap::{Arg, Command};
use geometry3d::{Point3D, Ray3D, Vector3D};
use rendering::d_coefficients::DCFactory;

fn main() {
    let matches = Command::new("SIMPLE Solar Simulation")
        .version("0.1 (but it is still awesome!)")
        .author("(c) German Molina")
        .about("A Climate Based Daylight Simulation tool")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("SIMPLE or Radiance file")
                .help("This is the SIMPLE Model or a Radiance file")
                .takes_value(true)
                .required(true),                
        )
        .arg(
            Arg::new("weather")
                .short('w')
                .long("weather_file")
                .value_name("EPW File")
                .help("This is an EPW weather file")
                .takes_value(true)
                .required(true),                
        )
        .get_matches();

    let input_file = matches.value_of("input").unwrap();
    let _weather_file = "asd"; //matches.value_of("weather").unwrap();

    let mut scene = if input_file.ends_with(".rad") {
        Scene::from_radiance(input_file.to_string())
    } else if input_file.ends_with(".simple") || input_file.ends_with(".spl") {
        panic!("Reading SIMPLE models is still not suppoerted")
    } else {
        eprintln!(
            "Don't know how to read file '{}'... only .rad is supported for now",
            input_file
        );
        std::process::exit(1);
    };

    scene.build_accelerator();

    // Setup sensors
    let up = Vector3D::new(0., 0., 1.);
    let rays = vec![
        Ray3D {
            origin: Point3D::new(2., 0.5, 0.8),
            direction: up,
        },
        Ray3D {
            origin: Point3D::new(2., 2.5, 0.8),
            direction: up,
        },
        Ray3D {
            origin: Point3D::new(2., 5.5, 0.8),
            direction: up,
        },
    ];

    eprintln!("Ready to calc!... # Surface = {}", scene.triangles.len());

    let mf = 1;
    let factory = DCFactory {
        max_depth: 2,
        n_ambient_samples: 50000,
        reinhart: ReinhartSky::new(mf),
        ..DCFactory::default()
    };

    let dc_matrix = factory.calc_dc(&rays, &scene);
    let dc_matrix = rendering::colour_matrix::colour_matrix_to_luminance(&dc_matrix);
    println!("Matrix = {}", dc_matrix);
}

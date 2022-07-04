
use rendering::{Float,  colour_matrix, ColourMatrix, Scene, DCFactory};
use geometry3d::{Point3D, Ray3D, Vector3D};
use validate::{valid, ScatterValidator, Validate, Validator};


fn flatten_matrix(m: &ColourMatrix)->Vec<Float>{
    let (nrows, ncols) = m.size();
    let mut v : Vec<Float> = Vec::with_capacity(nrows*ncols);
    for row in 0..nrows {
        for col in 0..ncols {
            let value = m.get(row, col).unwrap().radiance();
            v.push(value);
        }
    }
    v
}

fn load_rays(filename: &str) -> Vec<Ray3D> {
    let s = std::fs::read_to_string(filename).unwrap();
    s.lines()
        .map(|line| {
            let a: Vec<f64> = line
                .split_ascii_whitespace()
                .into_iter()
                .map(|x| x.parse::<Float>().unwrap())
                .collect();

            Ray3D {
                origin: Point3D::new(a[0], a[1], a[2]),
                direction: Vector3D::new(a[3], a[4], a[5]).get_normalized(),
            }
        })
        .collect()
}

fn load_expected_results(filename: String)->Vec<Float>{
    let path = std::path::Path::new(&filename);
    let m = colour_matrix::read_colour_matrix(path).unwrap();
    
    flatten_matrix(&m)
}

fn get_simple_results(dir: &str, max_depth: usize) -> (Vec<Float>, Vec<Float>) {
    let mut scene = Scene::from_radiance(format!("./tests/dc/{dir}/scene.rad"));
    scene.build_accelerator();

    let integrator = DCFactory {
        n_ambient_samples: 51020,        
        max_depth,
        limit_weight: 1e-9,
        .. DCFactory::default()
    };
        
    let rays = load_rays("./tests/points.pts");
    let found_matrix = integrator.calc_dc(&rays, &scene);
    let found = flatten_matrix(&found_matrix);

    let expected = if max_depth == 0 {
        load_expected_results(format!("./tests/dc/{dir}/direct_results.txt"))
    }else{
        load_expected_results(format!("./tests/dc/{dir}/global_results.txt"))
    };
    
    (expected, found)
}

/// Calculate the Daylight Coefficients in a room. 
#[valid(Room)]
fn room_global()->Box<dyn Validate>{
    let (expected, found) = get_simple_results("room", 12);

    let v = ScatterValidator {                
        units: Some("cd/m2"),
        expected,
        found,
        ..validate::ScatterValidator::default()
    };
    Box::new(v)
}


/// Calculate the Daylight Coefficients in a room, with zero bounces 
#[valid(Room Direct)]
fn room_direct()->Box<dyn Validate>{
    let (expected, found) = get_simple_results("room", 0);

    let v = ScatterValidator {                
        units: Some("cd/m2"),
        expected,
        found,
        ..validate::ScatterValidator::default()
    };
    Box::new(v)
}

#[test]
fn validate_dc(){
    // cargo test --release --features parallel --package rendering --test validate_dc -- validate_dc --exact --nocapture
    let mut validator = Validator::new("Validate Time series", "./docs/validation/daylight_coefficient.html");

    validator.push(room_global());
    validator.push(room_direct());
    
    

    validator.validate().unwrap();
}
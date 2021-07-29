use crate::material::{Light, Metal, Mirror, Plastic};
// use crate::sampleable_trait::Sampleable;
use crate::scene::Scene;

use geometry3d::distant_source3d::DistantSource3D;
use geometry3d::loop3d::Loop3D;
use geometry3d::point3d::Point3D;
use geometry3d::polygon3d::Polygon3D;
use geometry3d::sphere3d::Sphere3D;
use geometry3d::vector3d::Vector3D;

use std::fs;

#[derive(Default)]
struct Scanner {
    current_char_index: usize,
    is_done: bool,
    modifiers: Vec<String>,
    line: usize,
}

impl Scanner {
    fn error_here(&self, msg: String) {
        panic!("Error at line {}: {}", self.line, msg)
    }

    fn get_modifier_index(&self, name: &String) -> usize {
        for (i, mod_name) in self.modifiers.iter().enumerate() {
            if name == mod_name {
                return i;
            }
        }
        self.error_here(format!(
            "Unknown modifier '{}' in the scene ... modifiers are {:?}",
            name, self.modifiers
        ));
        unreachable!();
    }

    fn consume_whitespace(&mut self, source: &[u8]) -> bool {
        if self.is_done {
            return false;
        }

        if source[self.current_char_index].is_ascii_whitespace() {
            self.consume_char(source)
        } else {
            false
        }
    }

    fn consume_non_white(&mut self, source: &[u8]) -> bool {
        if self.is_done {
            return false;
        }
        if source[self.current_char_index].is_ascii_whitespace() {
            false
        } else {
            self.consume_char(source)
        }
    }

    fn consume_char(&mut self, source: &[u8]) -> bool {
        if self.is_done {
            return false;
        }
        if source[self.current_char_index] == b'\n' {
            self.line += 1;
        }
        self.current_char_index += 1;
        if self.current_char_index == source.len() {
            self.is_done = true;
        }
        true
    }

    /// Advances until reaching the next token
    fn reach_next_token(&mut self, source: &[u8]) {
        loop {
            if !self.consume_whitespace(source) {
                break;
            }
        }
    }

    /// Retrieves a token and advances.
    fn consume_token(&mut self, source: &[u8]) -> String {
        self.reach_next_token(source);

        let start = self.current_char_index;
        loop {
            if !self.consume_non_white(source) {
                break;
            }
        }

        if start == self.current_char_index {
            "".to_string()
        } else {
            let ret = std::str::from_utf8(&source[start..self.current_char_index])
                .unwrap()
                .to_string();
            ret
        }
    }

    /// Consume object
    fn consume_object(&mut self, source: &[u8], scene: &mut Scene) {
        self.reach_next_token(source);
        if self.is_done {
            return;
        }

        let modifier = self.consume_token(source);
        if self.is_done {
            self.error_here(format!("Incorrect source... no data after 'modifier'"));
        }
        let object_type = self.consume_token(source);
        if self.is_done {
            self.error_here(format!("Incorrect source... no data after 'object_type'"));
        }
        let name = self.consume_token(source);
        if self.is_done {
            self.error_here(format!("Incorrect source... no data after 'name'"));
        }
        match object_type.as_bytes() {
            // modifiers
            b"plastic" => self.consume_plastic(source, scene, &modifier, &name),
            b"metal" => self.consume_metal(source, scene, &modifier, &name),
            b"light" => self.consume_light(source, scene, &modifier, &name),
            b"mirror" => self.consume_mirror(source, scene, &modifier, &name),

            // objects
            b"sphere" => self.consume_sphere(source, scene, &modifier, &name),
            b"source" => self.consume_source(source, scene, &modifier, &name),
            b"polygon" => self.consume_polygon(source, scene, &modifier, &name),
            _ => {
                self.error_here(format!("Unsupported/unknown object_type '{}'", object_type));
                unreachable!();
            }
        }
    }

    /// Consumes a Metal material
    fn consume_metal(
        &mut self,
        source: &[u8],
        scene: &mut Scene,
        _modifier: &String,
        name: &String,
    ) {
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "5".to_string());
        let red = self.consume_token(source).parse::<f64>().unwrap();
        let green = self.consume_token(source).parse::<f64>().unwrap();
        let blue = self.consume_token(source).parse::<f64>().unwrap();
        let specularity = self.consume_token(source).parse::<f64>().unwrap();
        let roughness = self.consume_token(source).parse::<f64>().unwrap();

        self.modifiers.push(name.clone());

        let metal = Metal {
            red,
            green,
            blue,
            specularity,
            roughness,
        };
        scene.push_material(Box::new(metal));
    }

    /// Consumes a Plastic material
    fn consume_plastic(
        &mut self,
        source: &[u8],
        scene: &mut Scene,
        _modifier: &String,
        name: &String,
    ) {
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "5".to_string());
        let red = self.consume_token(source).parse::<f64>().unwrap();
        let green = self.consume_token(source).parse::<f64>().unwrap();
        let blue = self.consume_token(source).parse::<f64>().unwrap();
        let specularity = self.consume_token(source).parse::<f64>().unwrap();
        let roughness = self.consume_token(source).parse::<f64>().unwrap();

        self.modifiers.push(name.clone());

        let plastic = Plastic {
            red,
            green,
            blue,
            specularity,
            roughness,
        };
        scene.push_material(Box::new(plastic));
    }

    /// Consumes a Light material
    fn consume_light(
        &mut self,
        source: &[u8],
        scene: &mut Scene,
        _modifier: &String,
        name: &String,
    ) {
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "3".to_string());
        let red = self.consume_token(source).parse::<f64>().unwrap();
        let green = self.consume_token(source).parse::<f64>().unwrap();
        let blue = self.consume_token(source).parse::<f64>().unwrap();

        self.modifiers.push(name.clone());

        let light = Light { red, green, blue };
        scene.push_material(Box::new(light));
    }

    /// Consumes a Light material
    fn consume_mirror(
        &mut self,
        source: &[u8],
        scene: &mut Scene,
        _modifier: &String,
        name: &String,
    ) {
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "3".to_string());
        let red = self.consume_token(source).parse::<f64>().unwrap();
        let green = self.consume_token(source).parse::<f64>().unwrap();
        let blue = self.consume_token(source).parse::<f64>().unwrap();

        self.modifiers.push(name.clone());

        let mirror = Mirror { red, green, blue };
        scene.push_material(Box::new(mirror));
    }

    /// Consumes a sphere
    fn consume_sphere(
        &mut self,
        source: &[u8],
        scene: &mut Scene,
        modifier: &String,
        _name: &String,
    ) {
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "4".to_string());
        let center_x = self.consume_token(source).parse::<f64>().unwrap();
        let center_y = self.consume_token(source).parse::<f64>().unwrap();
        let center_z = self.consume_token(source).parse::<f64>().unwrap();
        let radius = self.consume_token(source).parse::<f64>().unwrap();

        let sphere = Sphere3D::new(radius, Point3D::new(center_x, center_y, center_z));

        let mod_index = self.get_modifier_index(modifier);
        scene.push_object(mod_index, mod_index, Box::new(sphere));
    }

    /// Consumes a sphere
    fn consume_source(
        &mut self,
        source: &[u8],
        scene: &mut Scene,
        modifier: &String,
        _name: &String,
    ) {
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "4".to_string());
        let dir_x = self.consume_token(source).parse::<f64>().unwrap();
        let dir_y = self.consume_token(source).parse::<f64>().unwrap();
        let dir_z = self.consume_token(source).parse::<f64>().unwrap();
        let mut angle = self.consume_token(source).parse::<f64>().unwrap();
        angle *= std::f64::consts::PI / 180.; // into radians
        let distant_source = DistantSource3D::new(Vector3D::new(dir_x, dir_y, dir_z), angle);

        let mod_index = self.get_modifier_index(modifier);
        scene.push_object(mod_index, mod_index, Box::new(distant_source));
    }
    /// Consumes a polygon
    fn consume_polygon(
        &mut self,
        source: &[u8],
        scene: &mut Scene,
        modifier: &String,
        _name: &String,
    ) {
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let t = self.consume_token(source);
        assert_eq!(t, "0".to_string());
        let mut vertex_n = self.consume_token(source).parse::<usize>().unwrap();
        if vertex_n % 3 != 0 {
            panic!("Incorrect Polygon... n%3 != 0")
        }

        let mut the_loop = Loop3D::new();

        while vertex_n > 0 {
            let x = self.consume_token(source).parse::<f64>().unwrap();
            let y = self.consume_token(source).parse::<f64>().unwrap();
            let z = self.consume_token(source).parse::<f64>().unwrap();
            the_loop.push(Point3D::new(x, y, z)).unwrap();
            vertex_n -= 3;
        }
        the_loop.close().unwrap();
        let polygon = Polygon3D::new(the_loop).unwrap();

        let mod_index = self.get_modifier_index(modifier);
        scene.push_object(mod_index, mod_index, Box::new(polygon));
    }
}

impl Scene {
    pub fn from_radiance(filename: String) -> Self {
        let src = fs::read(filename).unwrap();
        Scene::from_radiance_source(&src)
    }

    pub fn from_radiance_source(source: &[u8]) -> Self {
        let mut ret = Self::default();

        let mut scanner = Scanner::default();

        while !scanner.is_done {
            scanner.consume_object(source, &mut ret);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let scanner = Scanner::default();
        assert!(!scanner.is_done);
        assert_eq!(scanner.current_char_index, 0);
    }

    #[test]
    fn test_token() {
        let source: &[u8] = "car with wheels".as_bytes();
        let mut scanner = Scanner::default();

        scanner.reach_next_token(source);
        assert_eq!(scanner.current_char_index, 0);
        assert_eq!(source[scanner.current_char_index], b'c');

        //===
        let source: &[u8] = "    car with wheels".as_bytes();
        let mut scanner = Scanner::default();

        scanner.reach_next_token(source);
        assert_eq!(scanner.current_char_index, 4);
        assert_eq!(source[scanner.current_char_index], b'c');

        //consume tokens
        let token_1 = scanner.consume_token(source);
        assert_eq!(token_1, "car".to_string());
        assert_eq!(source[scanner.current_char_index], b' ');
        assert_eq!(scanner.current_char_index, 7);

        assert_eq!("with".to_string(), scanner.consume_token(source));
        assert_eq!("wheels".to_string(), scanner.consume_token(source));

        let end = scanner.consume_token(source);
        assert_eq!("".to_string(), end);
        assert!(scanner.is_done)
    }

    #[test]
    fn test_plastic() {
        let src = b"void plastic red
        0
        0
        5 0.3 0.05 0.05 0 0
        
        red sphere ball
        0
        0
        4   0 0 0.5 1.5";

        let mut scene = Scene::new();
        let mut scanner = Scanner::default();
        scanner.consume_object(src, &mut scene);
        assert_eq!(scene.n_materials(), 1);
        assert_eq!(scanner.modifiers.len(), 1);
        assert_eq!(scanner.modifiers[0], "red".to_string());
        assert_eq!(0, scanner.get_modifier_index(&"red".to_string()));
    }
}

//! [`PolyMesh`], an [`Object`] that reads and intersects with triangle meshes.
//!
//! ---
//!
//! krt - Ken's Raytracer - Coursework Edition. (C) Copyright 1993-2022.
//!
//! I've put a lot of time and effort into this code. For the last decade it's been used to
//! introduce hundreds of students at multiple universities to raytracing. It forms the basis of
//! your coursework but you are free to continue using/developing forever more. However, I ask that
//! you don't share the code or your derivitive versions publicly. In order to continue
//! to be used for coursework and in particular assessment it's important that versions containing
//! solutions are not searchable on the web or easy to download.
//!
//! If you want to show off your programming ability, instead of releasing the code, consider
//! generating an incredible image and explaining how you produced it.
//!
//! ---
//!
//! Rust reimplementation provided by a former student. This version is made available under the
//! same copyright and conditions as the original C++ implementation.

use crate::transform::Apply;
use crate::{
    hit::Hit,
    material::{falsecolour::FalseColour, Material},
    ray::Ray,
    transform::Transform,
    vertex::Vertex,
};
use std::fs;

use super::Object;

type TriangleIndex = [usize; 3];

/// `PolyMesh` reads and intersects with triangle meshes.
#[derive(Debug)]
pub struct PolyMesh {
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub vertices: Vec<Vertex>,
    pub triangle_indices: Vec<TriangleIndex>,
    pub smoothing: bool,
    material: Box<dyn Material>,
}

impl PolyMesh {
    /// This is the equivalent of the two-argument constructor from the C++ version.
    pub fn new(filename: &str, smoothing: bool) -> Self {
        let contents = fs::read_to_string(filename).expect("Should read the file");
        let mut lines = contents.lines();
        if lines.next() != Some("kcply") {
            panic!("Mesh file doesn't start with kcply");
        }

        // Read in number of vertices and faces
        let line2 = lines.next().expect("Line 2 is right");
        let vertex_count = line2
            .strip_prefix("element vertex ")
            .expect("ln2 starts with element vertex")
            .parse::<usize>()
            .expect("suffix is a number");
        let line3 = lines.next().expect("Line 3 is right");
        let triangle_count = line3
            .strip_prefix("element face ")
            .expect("ln3 starts with element face :)")
            .parse::<usize>()
            .expect("suffix is a number");

        let mut vertices = Vec::new();
        for ln in 0..vertex_count {
            let l = lines
                .next()
                .expect(format!("Valid line, ln {}", ln).as_str());
            let mut raw_coords = l.split_whitespace();
            let list = raw_coords.collect::<Vec<_>>();
            let v = Vertex::from_xyz(
                list[0].parse::<f32>().unwrap(),
                list[1].parse::<f32>().unwrap(),
                list[2].parse::<f32>().unwrap(),
            );
            vertices.push(v);
        }

        let mut triangle_indices = Vec::new();
        for ln in 0..triangle_count {
            let l = lines
                .next()
                .expect(format!("Valid line, ln {}", ln).as_str());
            let mut raw_verts = l.split_whitespace();
            let list = raw_verts.collect::<Vec<_>>();
            let v = [
                list[1].parse::<usize>().unwrap(),
                list[2].parse::<usize>().unwrap(),
                list[3].parse::<usize>().unwrap(),
            ];
            triangle_indices.push(v);
        }
        PolyMesh {
            vertex_count,
            triangle_count,
            vertices,
            triangle_indices,
            smoothing,
            material: Box::new(FalseColour::default()),
        }
    }
}

impl Object for PolyMesh {
    fn material(&self) -> &dyn Material {
        &*self.material
    }

    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let mut hits = vec![];
        let epsilon = 0.0000001;
        for &[i0, i1, i2] in &self.triangle_indices {
            let v0 = self.vertices[i0];
            let v1 = self.vertices[i1];
            let v2 = self.vertices[i2];

            let e1 = v1 - v0;
            let e2 = v2 - v0;
            let h = ray.direction.cross(e2);
            let a = e1.dot(h);
            // println!("{:?}", ray.direction);
            if a > -epsilon && a < epsilon {
                continue;
            }

            // println!("a is {a}");

            let f = 1. / a;
            let s = ray.position - v0;
            let u = f * s.dot(h);
            if u < 0. || u > 1. {
                continue;
            }
            // println!("u is {u}");

            let q = s.cross(e1);
            let v = f * ray.direction.dot(q);
            if v < 0. || u + v > 1. {
                continue;
            }
            // println!("v is {v}");

            let t = f * e2.dot(q);
            if t > epsilon {
                // ray intersects current triangle
                let entering = a <= -epsilon;
                // println!("{t}");
                let h = Hit {
                    t,
                    entering,
                    what: self,
                    position: ray.position + ray.direction * t,
                    normal: h,
                };
                hits.push(h);
            }
        }
        hits
    }

    fn apply_transform(&mut self, t: &Transform) {
        for v in &mut self.vertices {
            t.apply_to(v);
        }
    }
}

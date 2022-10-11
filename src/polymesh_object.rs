use crate::hit::Hit;
use crate::transform::{Apply, Transform};
use crate::vertex::Vertex;
use std::fs::File;
use std::io::BufRead;
use std::{fs, io};

type TriangleIndex = [usize; 3];

pub struct PolyMesh {
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub vertices: Vec<Vertex>,
    pub triangle_indices: Vec<TriangleIndex>,
    pub smoothing: bool,
}

impl PolyMesh {
    pub fn new(filename: &str, smoothing: bool) -> Self {
        let contents = fs::read_to_string(filename).expect("Should read the file");
        let mut lines = contents.lines();
        if lines.next() != Some("kcply") {
            PolyMesh::from_ply(filename, smoothing);
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
        }
    }
    pub fn from_ply(filename: &str, smoothing: bool) -> Self {
        todo!()
    }
    // pub fn intersection(&self, r: Ray) -> Vec<Hit> {
    //     todo!()
    // }
    pub fn apply_transform(&mut self, t: Transform) {
        for v in &mut self.vertices {
            t.apply_to(v);
        }
    }
}

use super::Object;
use crate::{
    hit::Hit,
    material::{falsecolour::FalseColour, Material},
    ray::Ray,
    Vertex,
};
use glam::Affine3A;
use std::fs;

type TriangleIndex = [usize; 3];

/// `PolyMesh` reads and intersects with triangle meshes.
#[derive(Debug)]
pub struct PolyMesh {
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub vertices: Vec<Vertex>,
    pub triangle_indices: Vec<TriangleIndex>,
    pub smoothing: bool,
    pub one_ind: bool,
    material: Box<dyn Material>,
}

impl PolyMesh {
    /// This is the equivalent of the two-argument constructor from the C++ version.
    pub fn new(filename: &str, smoothing: bool, one_ind: bool) -> Self {
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
            let raw_coords = l.split_whitespace();
            let list = raw_coords.collect::<Vec<_>>();
            let v = Vertex::new(
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
            let raw_verts = l.split_whitespace();
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
            one_ind,
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
            // if the .ply file is 1-indexed, adjust accordingly
            let n = if self.one_ind { 1 } else { 0 };

            let v0 = self.vertices[i0 - n];
            let v1 = self.vertices[i1 - n];
            let v2 = self.vertices[i2 - n];

            // implementing the MT algorithm which exploits Cramer's rule
            let e1 = v1 - v0;
            let e2 = v2 - v0;
            let h = ray.direction.cross(e2);
            let a = e1.dot(h);
            if a > -epsilon && a < epsilon {
                continue; // ray parallel to triangle
            }

            let f = 1. / a;
            let s = ray.position - v0;
            let u = f * s.dot(h);
            if u < 0. || u > 1. {
                continue; // condition from barycentric coords
            }

            let q = s.cross(e1);
            let v = f * ray.direction.dot(q);
            if v < 0. || u + v > 1. {
                continue; // condition from barycentric coords
            }

            let t = f * e2.dot(q);
            if t > epsilon {
                // successful ray intersection
                let plane_normal = e1.cross(e2);
                let entering = plane_normal.dot(ray.direction) < 0.;
                let h = Hit {
                    t,
                    entering,
                    object_hit: self,
                    position: ray.position + ray.direction * t,
                    normal: -plane_normal.normalize(),
                };
                hits.push(h);
            }
        }
        hits
    }

    fn apply_transform(&mut self, t: Affine3A) {
        let mut vertices = Vec::new();
        for v in &self.vertices {
            vertices.push(t.transform_point3a(*v));
        }
        self.vertices = vertices;
    }
}

use std::fs;

use glam::{Affine3A, Vec3A};

use super::Object;
use crate::{
    hit::Hit,
    material::{Material},
    ray::Ray,
    Vertex,
};

#[derive(Debug)]
pub struct MeshTriangle {
    pub normal: Vec3A,
    pub corners: [Corner; 3],
}

#[derive(Debug, Clone)]
pub struct Corner {
    pub pos: Vertex,
    pub normal: Vec3A,
}

#[derive(Debug)]
pub struct IntermediateTriangle {
    pub normal: Vec3A,
    pub corner_indices: [usize; 3],
}

#[derive(Debug)]
pub struct PolyMesh {
    pub triangles: Vec<MeshTriangle>,
    pub smoothing: bool,
    material: Box<dyn Material + Send + Sync>,
}

impl PolyMesh {
    pub fn new<M>(filename: &str, smoothing: bool, one_ind: bool, material: M) -> Self
    where
        M: Material + Send + Sync + 'static,
    {
        // if the .ply file is 1-indexed, adjust accordingly
        let n = if one_ind { 1 } else { 0 };
        let contents = fs::read_to_string(filename).expect("Should read the file");
        let mut lines = contents.lines();
        if lines.next() != Some("kcply") {
            panic!("Mesh file doesn't start with kcply");
        }

        // read in number of vertices and faces
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

        let mut intermed_triangles = Vec::new();
        let mut vertex_indices = vec![Vec::new(); vertex_count];
        for ln in 0..triangle_count {
            let l = lines
                .next()
                .expect(format!("Valid line, ln {}", ln).as_str());
            let raw_verts = l.split_whitespace();
            let list = raw_verts.collect::<Vec<_>>();
            let v = [
                list[1].parse::<usize>().unwrap() - n,
                list[2].parse::<usize>().unwrap() - n,
                list[3].parse::<usize>().unwrap() - n,
            ];
            for i in v {
                vertex_indices[i].push(ln);
            }

            let v0 = vertices[v[0]];
            let v1 = vertices[v[1]];
            let v2 = vertices[v[2]];
            let e1 = v1 - v0;
            let e2 = v2 - v0;
            let n = e1.cross(e2).normalize();

            intermed_triangles.push(IntermediateTriangle {
                normal: n,
                corner_indices: [v[0], v[1], v[2]],
            });
        }

        let mut triangles = Vec::new();
        for t in intermed_triangles.iter() {
            let corners = t.corner_indices.map(|c| {
                let neighbours = &*vertex_indices[c];
                let mut normal = Vec3A::default();
                for &n in neighbours {
                    normal += intermed_triangles[n].normal;
                }
                normal /= neighbours.len() as f32;
                Corner {
                    pos: vertices[c],
                    normal,
                }
            });
            triangles.push(MeshTriangle {
                normal: t.normal,
                corners,
            });
        }

        PolyMesh {
            triangles,
            smoothing,
            material: Box::new(material),
        }
    }
}

impl Object for PolyMesh {
    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let mut hits = vec![];
        let epsilon = 0.0000001;
        for triangle in &self.triangles {
            let [c0, c1, c2] = triangle.corners.clone();

            // implementing the MT algorithm which exploits Cramer's rule
            let e1 = c1.pos - c0.pos;
            let e2 = c2.pos - c0.pos;
            let h = ray.direction.cross(e2);
            let a = e1.dot(h);
            if a > -epsilon && a < epsilon {
                continue; // ray parallel to triangle
            }

            let f = 1. / a;
            let s = ray.position - c0.pos;
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
                let w = 1. - u - v;
                let mut plane_normal = if self.smoothing {
                    c0.normal * w + c1.normal * u + c2.normal * v
                } else {
                    -e1.cross(e2)
                };
                let entering = plane_normal.dot(ray.direction) < 0.;
                // flip normals for back face hits
                if !entering {
                    plane_normal = -plane_normal
                }
                let h = Hit {
                    t,
                    entering,
                    object_hit: self,
                    material: &*self.material,
                    position: ray.position + ray.direction * t,
                    normal: plane_normal.normalize(),
                    incident: ray.clone(),
                };
                hits.push(h);
            }
        }
        hits
    }

    fn apply_transform(&mut self, t: Affine3A) {
        for triangle in &mut self.triangles {
            triangle.normal = t.transform_vector3a(triangle.normal);
            for corner in &mut triangle.corners {
                corner.normal = t.transform_vector3a(corner.normal);
                corner.pos = t.transform_point3a(corner.pos);
            }
        }
    }
}

use glam::{Affine3A, Vec3A};

use crate::{
    hit::Hit,
    material::{normalshading::NormalShading, Material},
    object::Object,
    ray::Ray,
    Vertex,
};

#[derive(Debug)]
pub struct Triangle {
    pub normal: Vec3A,
    pub corners: [Vertex; 3],
    material: Box<dyn Material>,
}

impl Triangle {
    pub fn new(corners @ [v0, v1, v2]: [Vertex; 3]) -> Self {
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(e2).normalize();
        Triangle {
            normal,
            corners,
            material: Box::new(NormalShading::default()),
        }
    }
    pub fn new_with_material(
        corners @ [v0, v1, v2]: [Vertex; 3],
        material: Box<dyn Material>,
    ) -> Self {
        let e1 = v1 - v0;
        let e2 = v2 - v0;
        let normal = e1.cross(e2).normalize();
        Triangle {
            normal,
            corners,
            material,
        }
    }
}

impl Object for Triangle {
    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let epsilon = 0.0001;
        let [c0, c1, c2] = self.corners.clone();

        // implementing the MT algorithm which exploits Cramer's rule
        let e1 = c1 - c0;
        let e2 = c2 - c0;
        let h = ray.direction.cross(e2);
        let a = e1.dot(h);
        if a > -epsilon && a < epsilon {
            return Vec::new(); // ray parallel to triangle
        }

        let f = 1. / a;
        let s = ray.position - c0;
        let u = f * s.dot(h);
        if u < 0. || u > 1. {
            return Vec::new(); // condition from barycentric coords
        }

        let q = s.cross(e1);
        let v = f * ray.direction.dot(q);
        if v < 0. || u + v > 1. {
            return Vec::new(); // condition from barycentric coords
        }

        let t = f * e2.dot(q);
        if t > epsilon {
            // successful ray intersection
            let plane_normal = e1.cross(e2);
            let normal = if plane_normal.dot(ray.direction) < 0. {
                plane_normal
            } else {
                -plane_normal
            };
            vec![Hit {
                t,
                entering: true,
                object_hit: self,
                material: &*self.material,
                position: ray.position + ray.direction * t,
                normal: normal.normalize(),
                incident: ray.clone(),
            }]
        } else {
            Vec::new()
        }
    }

    fn apply_transform(&mut self, t: Affine3A) {
        self.normal = t.transform_vector3a(self.normal);
        for corner in &mut self.corners {
            *corner = t.transform_point3a(*corner);
        }
    }
}

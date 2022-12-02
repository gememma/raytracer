use glam::{Affine3A, Vec3A};

use crate::{
    hit::Hit,
    material::{normalshading::NormalShading, Material},
    object::Object,
    ray::Ray,
    Vertex,
};

#[derive(Debug)]
pub struct Plane {
    normal: Vec3A,
    pos: Vertex,
    material: Box<dyn Material>,
}

impl Plane {
    pub fn new(normal: Vec3A, pos: Vertex) -> Self {
        Self {
            normal: normal.normalize(),
            pos,
            material: Box::new(NormalShading::default()),
        }
    }
}

impl Object for Plane {
    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let epsilon = 0.0000001;
        let a = self.normal.dot(ray.direction);
        if a > -epsilon && a < epsilon {
            return Vec::new(); // ray parallel to plane
        }

        let w = self.pos - ray.position;
        let f = w.dot(self.normal);
        let t = f / a;

        vec![Hit {
            t,
            entering: self.normal.dot(ray.direction) < 0.,
            object_hit: self,
            material: &*self.material,
            position: ray.position + ray.direction * t,
            normal: self.normal,
            incident: ray.clone(),
        }]
    }

    fn apply_transform(&mut self, t: Affine3A) {
        self.pos = t.transform_point3a(self.pos);
        self.normal = t.transform_vector3a(self.normal);
    }
}

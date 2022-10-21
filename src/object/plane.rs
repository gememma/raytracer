use super::Object;
use crate::{
    hit::Hit,
    material::{normalshading::NormalShading, Material},
    ray::Ray,
};
use glam::Affine3A;

#[derive(Debug)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    material: Box<dyn Material>,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            a,
            b,
            c,
            d,
            material: Box::new(NormalShading::default()),
        }
    }
}

impl Object for Plane {
    fn material(&self) -> &dyn Material {
        &*self.material
    }

    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    fn intersection(&self, _ray: &Ray) -> Vec<Hit> {
        todo!()
    }

    fn apply_transform(&mut self, _transform: Affine3A) {
        todo!()
    }
}

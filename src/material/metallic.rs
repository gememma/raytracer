use glam::Vec3A;

use super::Material;
use crate::{
    colour::Colour,
    hit::Hit,
    ray::{Ray, Reflectable},
    scene::Scene,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Metallic {
    specular: Colour,
    power: f32,
}

impl Metallic {
    pub fn new(specular: Colour, power: f32) -> Self {
        Metallic { specular, power }
    }
}

impl Material for Metallic {
    fn compute_once(&self, viewer: Vec3A, hit: &Hit, recurse: usize, scene: &Scene) -> Colour {
        if recurse < 1 {
            return Colour::default();
        }
        let r = hit.incident.direction.normalize().reflect(hit.normal);
        self.specular
            * scene
                .raytrace(Ray::new(hit.position + 0.001 * r, r), recurse - 1, viewer)
                .0
    }

    fn compute_per_light(&self, _viewer: Vec3A, _hit: &Hit, _ldir: Vec3A) -> Colour {
        return Colour::default();
    }
}

use glam::Vec3A;

use crate::{colour::Colour, hit::Hit, material::Material, ray::Ray, scene::Scene};

#[derive(Clone, Debug, PartialEq)]
pub struct Dielectric {
    refractive_index: f32,
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Self {
        Dielectric { refractive_index }
    }

    fn refract(hit: &Hit, ratio: f32) -> Vec3A {
        let cos_theta = (-hit.incident.direction.normalize())
            .dot(hit.normal)
            .min(1.);
        let ref_perp = ratio * (hit.incident.direction.normalize() + cos_theta * hit.normal);
        let ref_para = -(1. - ref_perp.length_squared()).abs().sqrt() * hit.normal;
        ref_perp + ref_para
    }
}

impl Material for Dielectric {
    fn compute(&self, viewer: Vec3A, hit: &Hit, recurse: usize, scene: &Scene) -> Colour {
        let ratio = if hit.entering {
            1.0003 / self.refractive_index
        } else {
            self.refractive_index / 1.0003
        };
        let r = Dielectric::refract(hit, ratio);
        scene
            .raytrace(Ray::new(hit.position + 0.001 * r, r), recurse - 1, viewer)
            .0
    }
}

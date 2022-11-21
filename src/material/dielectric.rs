use glam::Vec3A;
use rand::random;

use crate::{
    colour::Colour,
    hit::Hit,
    material::Material,
    ray::{Ray, Reflectable},
    scene::Scene,
};

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

    fn reflectance(cos: f32, index: f32) -> f32 {
        // the Schlick approximation for reflectance
        let r0 = ((1. - index) / (1. + index)).powi(2);
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn compute(&self, viewer: Vec3A, hit: &Hit, recurse: usize, scene: &Scene) -> Colour {
        // TODO: change to stochastic ray generation to avoid branching
        let ratio = if hit.entering {
            1.0003 / self.refractive_index
        } else {
            self.refractive_index / 1.0003
        };
        let cos_theta = (-hit.incident.direction.normalize())
            .dot(hit.normal)
            .min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();
        let refl_probability = Dielectric::reflectance(cos_theta, ratio);
        let r = hit.incident.direction.reflect(hit.normal);
        if ratio * sin_theta > 1. {
            // cannot refract, total internal reflection occurs
            scene
                .raytrace(Ray::new(hit.position + 0.001 * r, r), recurse - 1, viewer)
                .0
        } else {
            let r1 = Dielectric::refract(hit, ratio);
            (scene
                .raytrace(Ray::new(hit.position + 0.001 * r, r), recurse - 1, viewer)
                .0
                * refl_probability)
                + (scene
                    .raytrace(Ray::new(hit.position + 0.001 * r1, r1), recurse - 1, viewer)
                    .0
                    * (1. - refl_probability))
        }
    }
}

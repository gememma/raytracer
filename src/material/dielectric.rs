use glam::Vec3A;
use rand::random;

use crate::{
    colour::Colour,
    hit::Hit,
    material::Material,
    photonmap::Interaction,
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
        let event = self.interact(hit);
        if let Interaction::Transmitted { ray, attenuation } = event {
            scene.raytrace(ray, recurse - 1, viewer).0
        } else if let Interaction::Reflected { ray, attenuation } = event {
            scene.raytrace(ray, recurse - 1, viewer).0
        } else {
            unreachable!()
        }
    }

    fn interact(&self, hit: &Hit) -> Interaction {
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
        if ratio * sin_theta <= 1. && random::<f32>() > refl_probability {
            let r = Dielectric::refract(hit, ratio);
            let ray = Ray::new(hit.position + 0.001 * r, r);
            Interaction::Transmitted {
                ray,
                attenuation: Colour::from_rgb(1., 1., 1.),
            }
        } else {
            let r = hit.incident.direction.reflect(hit.normal);
            let ray = Ray::new(hit.position + 0.001 * r, r);
            Interaction::Reflected {
                ray,
                attenuation: Colour::from_rgb(1., 1., 1.),
            }
        }
    }
}
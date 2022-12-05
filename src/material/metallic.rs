use glam::Vec3A;

use super::Material;
use crate::{
    colour::Colour,
    hit::Hit,
    photonmap::Interaction,
    ray::{Ray, Reflectable},
    scene::Scene,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Metallic {
    colour: Colour,
    power: f32,
}

impl Metallic {
    pub fn new(specular: Colour, power: f32) -> Self {
        Metallic {
            colour: specular,
            power,
        }
    }
}

impl Material for Metallic {
    fn compute(&self, viewer: Vec3A, hit: &Hit, recurse: usize, scene: &Scene) -> Colour {
        if recurse < 1 {
            return Colour::default();
        }
        if let Interaction::Reflected { ray, attenuation } = self.interact(hit) {
            attenuation * scene.raytrace(ray, recurse - 1, viewer).0
        } else {
            unreachable!()
        }
    }

    fn interact(&self, hit: &Hit) -> Interaction {
        let r = hit.incident.direction.normalize().reflect(hit.normal);
        let ray = Ray::new(hit.position + 0.001 * r, r);
        Interaction::Reflected {
            ray,
            attenuation: self.colour,
        }
    }
}

use glam::Vec3A;

use super::Material;
use crate::{
    colour::Colour,
    hit::Hit,
    light::point::random_in_unit_sphere,
    photonmap::{Interaction, PhotonMap},
    ray::{Ray, Reflectable},
    scene::Scene,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Metallic {
    colour: Colour,
    roughness: f32,
}

impl Metallic {
    pub fn new(specular: Colour, roughness: f32) -> Self {
        Metallic {
            colour: specular,
            roughness,
        }
    }
}

impl Material for Metallic {
    fn compute(
        &self,
        viewer: Vec3A,
        hit: &Hit,
        recurse: usize,
        scene: &Scene,
        pmap: &PhotonMap,
    ) -> Colour {
        if recurse < 1 {
            return Colour::default();
        }
        if let Interaction::Reflected { ray, attenuation } = self.interact(hit) {
            attenuation * scene.raytrace(ray, recurse - 1, viewer, pmap).0
        } else {
            unreachable!()
        }
    }

    fn interact(&self, hit: &Hit) -> Interaction {
        let r = (hit.incident.direction.normalize().reflect(hit.normal)
            + self.roughness * random_in_unit_sphere())
        .normalize();
        let ray = Ray::new(hit.position + 0.001 * r, r);
        Interaction::Reflected {
            ray,
            attenuation: self.colour,
        }
    }
}

use glam::Vec3A;

use super::Material;
use crate::{colour::Colour, hit::Hit, ray::Reflectable, scene::Scene};

#[derive(Clone, Debug, PartialEq)]
pub struct Phong {
    ambient: Colour,
    diffuse: Colour,
    specular: Colour,
    power: f32,
}

impl Phong {
    pub fn new(ambient: Colour, diffuse: Colour, specular: Colour, power: f32) -> Self {
        Phong {
            ambient,
            diffuse,
            specular,
            power,
        }
    }
}

impl Material for Phong {
    // ambient term
    fn compute_once(&self, _viewer: Vec3A, _hit: &Hit, _recurse: usize, _scene: &Scene) -> Colour {
        let ambient_intensity = 0.3;
        self.ambient * ambient_intensity
    }

    // diffuse and specular terms
    fn compute_per_light(&self, viewer: Vec3A, hit: &Hit, ldir: Vec3A) -> Colour {
        let dotprod = hit.normal.dot(ldir);
        let diffuse = if dotprod < 0. {
            Colour::default()
        } else {
            self.diffuse * dotprod
        };

        let r = -ldir.reflect(hit.normal);
        let specular = self.specular * (r.dot(-viewer).powf(self.power));
        diffuse + specular
    }
}

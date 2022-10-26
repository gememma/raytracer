use crate::{colour::Colour, hit::Hit};
use glam::Vec3A;

use super::Material;

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
    fn compute_once(&self, _viewer: Vec3A, _hit: &Hit, _recurse: usize) -> Colour {
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

        // TODO: move into reflection function
        let d = hit.normal.dot(-ldir) * 2.;
        let r = Vec3A::new(
            -ldir.x - d * hit.normal.x,
            -ldir.y - d * hit.normal.y,
            -ldir.z - d * hit.normal.z,
        );
        let specular = self.specular * (r.dot(-viewer).powf(self.power));
        diffuse + specular
    }
}

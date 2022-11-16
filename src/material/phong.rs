use glam::Vec3A;

use super::Material;
use crate::{
    colour::Colour,
    hit::Hit,
    ray::{Ray, Reflectable},
    scene::Scene,
};

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

impl Material for Phong {
    // ambient term
    fn compute(&self, viewer: Vec3A, hit: &Hit, _recurse: usize, scene: &Scene) -> Colour {
        let ambient_intensity = 0.3;
        let mut colour = self.ambient * ambient_intensity;

        for light in &scene.light_list {
            // ldir is direction towards the light
            let (ldir, mut lit) = light.get_direction(hit.position);

            if ldir.dot(hit.normal) < 0. {
                // Light is facing wrong way.
                lit = false;
            }

            if lit {
                lit = !scene
                    .shadowtrace(&Ray::new(hit.position + 0.0001 * ldir, ldir), f32::INFINITY);
            }

            if lit {
                let intensity = light.get_intensity(hit.position);
                colour += intensity * self.compute_per_light(viewer, &hit, ldir);
            }
        }
        colour
    }
}

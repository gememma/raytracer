use glam::Vec3A;
use rand::{random, Rng};

use super::Material;
use crate::{
    colour::Colour,
    hit::Hit,
    photonmap::{Interaction, PhotonMap},
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
    fn compute(
        &self,
        viewer: Vec3A,
        hit: &Hit,
        _recurse: usize,
        scene: &Scene,
        _pmap: &PhotonMap,
    ) -> Colour {
        let ambient_intensity = 0.3;
        let mut colour = self.ambient * ambient_intensity;

        for light in &scene.light_list {
            // ldir is direction towards the light
            let (ldir, mut lit) = light.get_direction(hit.position);

            if ldir.dot(hit.normal) < 0. {
                // light is facing wrong way
                lit = false;
            }

            if lit {
                // check for objects between position and light
                lit = !scene
                    .shadow_trace(&Ray::new(hit.position + 0.0001 * ldir, ldir), f32::INFINITY);
            }

            if lit {
                let intensity = light.get_intensity(hit.position);
                colour += intensity * self.compute_per_light(viewer, &hit, ldir);
            }
        }
        colour
    }

    fn interact(&self, hit: &Hit) -> Interaction {
        let diffuse_p = (self.diffuse.r + self.diffuse.g + self.diffuse.b) / 3.;
        if random::<f32>() > diffuse_p {
            Interaction::Absorbed
        } else {
            let r = random_in_unit_hemisphere(hit.normal);
            let ray = Ray::new(hit.position + 0.0001 * r, r);
            Interaction::Reflected {
                ray,
                attenuation: self.diffuse,
            }
        }
    }
}

pub fn random_in_unit_hemisphere(normal: Vec3A) -> Vec3A {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3A::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if p.length_squared() < 1. || p.dot(normal) > 0. {
            return p;
        }
    }
}

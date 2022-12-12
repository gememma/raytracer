use glam::Vec3A;
use rand::{random, Rng};

use crate::{
    colour::Colour,
    hit::Hit,
    material::Material,
    photonmap::{Interaction, PhotonMap},
    ray::Ray,
    scene::Scene,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Diffuse {
    colour: Colour,
}

impl Diffuse {
    pub fn new(colour: Colour) -> Self {
        Diffuse { colour }
    }
}

impl Material for Diffuse {
    fn compute(
        &self,
        _viewer: Vec3A,
        hit: &Hit,
        recurse: usize,
        scene: &Scene,
        _pmap: &PhotonMap,
    ) -> Colour {
        if recurse < 1 {
            return Colour::default();
        }

        let mut colour = Colour::from_rgb(0., 0., 0.);
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
                let dotprod = hit.normal.dot(ldir);
                let diffuse = if dotprod < 0. {
                    Colour::default()
                } else {
                    self.colour * dotprod
                };
                colour += intensity * diffuse;
            }
        }

        colour
    }

    fn interact(&self, hit: &Hit) -> Interaction {
        // darker objects are more likely to absorb light
        let diffuse_p = (self.colour.r + self.colour.g + self.colour.b) / 3.;
        if random::<f32>() > diffuse_p {
            Interaction::Absorbed
        } else {
            let r = random_in_unit_hemisphere(hit.normal);
            let ray = Ray::new(hit.position + 0.0001 * r, r);
            Interaction::Reflected {
                ray,
                attenuation: self.colour,
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

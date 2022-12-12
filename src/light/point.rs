use glam::Vec3A;
use rand::Rng;

use crate::{
    colour::Colour,
    light::Light,
    photonmap::{Photon, Type},
    ray::Ray,
    Vertex,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub position: Vertex,
    pub intensity: Colour,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            position: Vertex::default(),
            intensity: Colour::default(),
        }
    }
}

impl Point {
    pub fn new(position: Vertex, intensity: Colour) -> Self {
        Self {
            position,
            intensity,
        }
    }
}

impl Light for Point {
    fn get_direction(&self, surface: Vertex) -> (Vec3A, bool) {
        // point lights emit in all directions, so this boolean is always true
        ((self.position - surface).normalize(), true)
    }

    fn get_intensity(&self, _surface: Vertex) -> Colour {
        self.intensity
    }

    fn generate_photon(&self) -> Photon {
        let direction = random_in_unit_sphere();
        Photon {
            ray: Ray::new(self.position, direction),
            colour: self.intensity,
            type_: Type::Direct,
        }
    }
}

pub fn random_in_unit_sphere() -> Vec3A {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3A::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        );
        if p.length_squared() < 1. {
            return p;
        }
    }
}

use glam::Vec3A;

use super::Light;
use crate::{colour::Colour, photonmap::Photon, Vertex};

#[derive(Clone, Debug, PartialEq)]
pub struct Directional {
    pub direction: Vec3A,
    pub intensity: Colour,
}

impl Default for Directional {
    fn default() -> Self {
        Self {
            direction: Vec3A::default(),
            intensity: Colour::default(),
        }
    }
}

impl Directional {
    pub fn new(direction: Vec3A, intensity: Colour) -> Self {
        Self {
            direction: direction.normalize(),
            intensity,
        }
    }
}

impl Light for Directional {
    fn get_direction(&self, _surface: Vertex) -> (Vec3A, bool) {
        (-self.direction, true)
    }

    fn get_position(&self) -> Option<Vertex> {
        // directional lights have no position
        None
    }

    fn get_intensity(&self, _surface: Vertex) -> Colour {
        self.intensity
    }

    fn generate_photon(&self) -> Photon {
        // cannot generate a photon an infinite distance away
        unimplemented!()
    }
}

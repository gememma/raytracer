use std::fmt::Debug;

use glam::Vec3A;

use crate::{colour::Colour, photonmap::Photon, Vertex};

pub mod directional;
pub mod point;

pub trait Light: Debug {
    // Get direction towards the light from the surface point
    fn get_direction(&self, surface: Vertex) -> (Vec3A, bool);

    fn get_intensity(&self, surface: Vertex) -> Colour;

    fn generate_photon(&self) -> Photon;
}

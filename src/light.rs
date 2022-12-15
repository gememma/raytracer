use std::fmt::Debug;

use glam::Vec3A;

use crate::{colour::Colour, photonmap::Photon, Vertex};

pub mod directional;
pub mod point;

pub trait Light: Debug {
    // get direction towards the light from the surface point
    fn get_direction(&self, surface: Vertex) -> (Vec3A, bool);

    // get position of the light if it has one
    fn get_position(&self) -> Option<Vertex>;

    // get light intensity at given location
    fn get_intensity(&self, surface: Vertex) -> Colour;

    // emit a new photon
    fn generate_photon(&self) -> Photon;
}

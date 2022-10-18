use crate::{colour::Colour, Vertex};
use glam::Vec3A;
use std::fmt::Debug;

pub mod directional;

pub trait Light: Debug {
    // Get the direction towards the light at the point on the surface, and a `bool` indicating
    // where the surface is in comparison to the light: `true` if the surface is in front of the
    // light, and `false` if it's behind and not illuminated.
    fn get_direction(&self, surface: Vertex) -> (Vec3A, bool);

    fn get_intensity(&self, surface: Vertex) -> Colour;
}

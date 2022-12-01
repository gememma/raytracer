use glam::Vec3A;

use crate::{colour::Colour, light::Light, Vertex};

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
        ((self.position - surface).normalize(), true)
    }

    fn get_intensity(&self, _surface: Vertex) -> Colour {
        self.intensity
    }
}

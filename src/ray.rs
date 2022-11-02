use glam::Vec3A;

use crate::Vertex;

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub position: Vertex,
    pub direction: Vec3A,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            position: Vertex::default(),
            direction: Vec3A::default(),
        }
    }
}

impl Ray {
    pub fn new(position: Vertex, direction: Vec3A) -> Self {
        Self {
            position,
            direction,
        }
    }
}

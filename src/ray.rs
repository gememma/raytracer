use glam::Vec3A;

use crate::Vertex;

/// Ray is a struct to store and manipulate 3D rays.
#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub position: Vertex,
    pub direction: Vec3A,
}

impl Default for Ray {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self {
            position: Vertex::default(),
            direction: Vec3A::default(),
        }
    }
}

impl Ray {
    /// This is the equivalent of the two-argument constructor from the C++ version.
    pub fn new(position: Vertex, direction: Vec3A) -> Self {
        Self {
            position,
            direction,
        }
    }
}

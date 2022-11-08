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

pub trait Reflectable {
    fn reflect(&self, normal: Vec3A) -> Self;
}

impl Reflectable for Vec3A {
    fn reflect(&self, normal: Vec3A) -> Self {
        // function expects self(incident ray) to point towards surface
        let d = normal.dot(self.clone()) * 2.;
        Vec3A::new(
            self.x - d * normal.x,
            self.y - d * normal.y,
            self.z - d * normal.z,
        )
    }
}

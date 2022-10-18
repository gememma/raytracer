use glam::Vec3A;
use std::fmt;

use crate::{object::Object, Vertex};

#[derive(Clone, Debug)]
pub struct Hit<'obj> {
    pub t: f32,
    pub entering: bool,
    pub object_hit: &'obj dyn Object,
    pub position: Vertex,
    pub normal: Vec3A,
}

impl fmt::Display for Hit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            // The double brackets in {{ and }} are intentional: because Rust uses {} to represent
            // an interpolated value in a format string, {{ means "print a single { character".
            "Hit{{,[{},{},{}],[{},{},{}]}}",
            self.position.x,
            self.position.y,
            self.position.z,
            self.normal.x,
            self.normal.y,
            self.normal.z
        )
    }
}

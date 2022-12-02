use glam::Vec3A;

use crate::{material::Material, object::Object, ray::Ray, Vertex};

#[derive(Clone, Debug)]
pub struct Hit<'obj> {
    pub t: f32,
    pub entering: bool,
    pub object_hit: &'obj dyn Object,
    pub material: &'obj dyn Material,
    pub position: Vertex,
    pub normal: Vec3A,
    pub incident: Ray,
}

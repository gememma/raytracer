use glam::Vec3A;

use super::Material;
use crate::{colour::Colour, hit::Hit, scene::Scene};

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct NormalShading;

impl Material for NormalShading {
    fn compute(&self, _viewer: Vec3A, hit: &Hit, _recurse: usize, _scene: &Scene) -> Colour {
        Colour::from_rgb(
            (hit.normal.x + 1.) * 0.5,
            (hit.normal.y + 1.) * 0.5,
            (-hit.normal.z + 1.) * 0.5,
        )
    }
}

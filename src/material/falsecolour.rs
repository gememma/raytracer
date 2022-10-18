use crate::{colour::Colour, hit::Hit};
use glam::Vec3A;

use super::Material;

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FalseColour;

impl Material for FalseColour {
    fn compute_once(&self, _viewer: Vec3A, hit: &Hit, _recurse: usize) -> Colour {
        Colour::from_rgb(
            (hit.normal.x + 1.) * 0.5,
            (hit.normal.y + 1.) * 0.5,
            (-hit.normal.z + 1.) * 0.5,
        )
    }

    fn compute_per_light(&self, _viewer: Vec3A, _hit: &Hit, _ldir: Vec3A) -> Colour {
        Colour::from_rgb(0., 0., 0.)
    }
}

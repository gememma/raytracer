use std::fmt::Debug;

use glam::Vec3A;

use crate::{colour::Colour, hit::Hit};

pub mod normalshading;
pub mod phong;

pub trait Material: Debug + Send + Sync {
    fn compute_once(&self, viewer: Vec3A, hit: &Hit, recurse: usize) -> Colour;

    fn compute_per_light(&self, viewer: Vec3A, hit: &Hit, ldir: Vec3A) -> Colour;
}

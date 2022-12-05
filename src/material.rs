use std::fmt::Debug;

use glam::Vec3A;

use crate::{colour::Colour, hit::Hit, photonmap::Interaction, scene::Scene};

pub mod dielectric;
pub mod metallic;
pub mod normalshading;
pub mod phong;

pub trait Material: Debug + Send + Sync {
    fn compute(&self, viewer: Vec3A, hit: &Hit, recurse: usize, scene: &Scene) -> Colour;
    fn interact(&self, hit: &Hit) -> Interaction;
}

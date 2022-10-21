use crate::{hit::Hit, material::Material, ray::Ray};

use glam::Affine3A;
use std::fmt::Debug;

pub mod plane;
pub mod polymesh;
pub mod sphere;

pub trait Object: Debug {
    fn material(&self) -> &dyn Material;

    fn set_material(&mut self, material: Box<dyn Material>);

    fn intersection(&self, ray: &Ray) -> Vec<Hit>;

    fn apply_transform(&mut self, t: Affine3A);
}

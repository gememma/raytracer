use std::fmt::Debug;

use glam::Affine3A;

use crate::{hit::Hit, material::Material, ray::Ray};

pub mod csg;
pub mod plane;
pub mod polymesh;
pub mod quadratic;
pub mod sphere;
pub mod triangle;

pub trait Object: Debug {
    fn set_material(&mut self, material: Box<dyn Material + Send + Sync>);

    fn intersection(&self, ray: &Ray) -> Vec<Hit>;

    fn apply_transform(&mut self, t: Affine3A);
}

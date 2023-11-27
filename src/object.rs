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
    // find intersections between the object and the ray if they exist
    fn intersection(&self, ray: &Ray) -> Vec<Hit>;

    // transform the object
    fn apply_transform(&mut self, t: Affine3A);
}

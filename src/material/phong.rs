//! The [`Phong`] material, a [`Material`] that implements the simple Phong surface illumination
//! model.
//!
//! ---
//!
//! krt - Ken's Raytracer - Coursework Edition. (C) Copyright 1993-2022.
//!
//! I've put a lot of time and effort into this code. For the last decade it's been used to
//! introduce hundreds of students at multiple universities to raytracing. It forms the basis of
//! your coursework but you are free to continue using/developing forever more. However, I ask that
//! you don't share the code or your derivitive versions publicly. In order to continue
//! to be used for coursework and in particular assessment it's important that versions containing
//! solutions are not searchable on the web or easy to download.
//!
//! If you want to show off your programming ability, instead of releasing the code, consider
//! generating an incredible image and explaining how you produced it.
//!
//! ---
//!
//! Rust reimplementation provided by a former student. This version is made available under the
//! same copyright and conditions as the original C++ implementation.

use crate::{colour::Colour, hit::Hit, ray::Ray, vector::Vector};

use super::Material;

/// `Phong` implements the simple Phong surface illumination model.
#[derive(Clone, Debug, PartialEq)]
pub struct Phong {}

impl Phong {
    /// This is the equivalent of the four-argument constructor from the C++ version.
    pub fn new(_ambient: Colour, _diffuse: Colour, _specular: Colour, _power: f32) -> Self {
        todo!("you must implement the parameterised constructor for Phong")
    }
}

impl Default for Phong {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        todo!("you must implement the default constructor for Phong")
    }
}

impl Material for Phong {
    /// The `compute_once()` method supplies the ambient term.
    fn compute_once(&self, _viewer: &Ray, _hit: &Hit, _recurse: usize) -> Colour {
        todo!("you must implement Material for Phong")
    }

    /// The `compute_per_light()` method supplies the diffuse and specular terms.
    fn compute_per_light(&self, _viewer: Vector, _hit: &Hit, _ldir: Vector) -> Colour {
        todo!("you must implement Material for Phong")
    }
}

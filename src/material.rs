//! The [`Material`] trait, and [`Compound`][compound::Compound],
//! [`FalseColour`][falsecolour::FalseColour] and [`Phong`][phong::Phong] implementations.
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

use std::fmt::Debug;

pub mod compound;
pub mod falsecolour;
pub mod phong;

/// `Material` is the trait that all materials must conform to.
///
/// You will need to add additional methods to support photon-mapping.
pub trait Material: Debug {
    /// `compute_once()` is called once per intersection.
    fn compute_once(&self, viewer: &Ray, hit: &Hit, recurse: usize) -> Colour;

    /// `compute_per_light()` is called for each light that reaches a surface.
    fn compute_per_light(&self, viewer: Vector, hit: &Hit, ldir: Vector) -> Colour;
}

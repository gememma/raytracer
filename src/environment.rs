//! [`Environment`], a trait for anything that can perform raytracing (like
//! [`Scene`][crate::scene::Scene]).
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

use crate::vertex::Vertex;
use crate::{colour::Colour, ray::Ray};

/// Environment is the trait implemented by anything that can perform raytracing. We use this in
/// [`Material`][crate::material::Material] implementations to do recursion as that allows
/// [`Scene`][crate::scene::Scene], which implements this trait, to depend (indirectly) on
/// `Material`.
pub trait Environment {
    /// Shoot a [`Ray`] into the environment and get the colour and depth. `recurse` indicates the
    /// level of recursion permitted.
    fn raytrace(&self, ray: Ray, recurse: usize, viewer: Vertex) -> (Colour, f32);

    /// Raytrace a shadow ray. Returns `true` if intersection found between 0 and `limit` along
    /// ray.
    fn shadowtrace(&self, ray: &Ray, limit: f32) -> bool;
}

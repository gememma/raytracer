//! The [`Light`] trait, and the [`DirectionalLight`][directional::DirectionalLight]
//! implementation.
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

use crate::{colour::Colour, vector::Vector, vertex::Vertex};

use std::fmt::Debug;

pub mod directional;

/// `Light` is the trait that all lights must conform to.
///
/// You will need to add additional methods to this trait to support photon-mapping.
///
/// ## Rust
///
/// Unlike in C++, `Light` does not also form a linked list of lights (this is for several reasons,
/// not the least of which being that linked lists are difficult to represent in safe Rust code).
/// Instead, you should use a [`Vec`] --- most likely a [`Vec<Box<dyn Light>>`] if you need to
/// store a heterogeneous list of lights.
pub trait Light: Debug {
    /// Get the direction towards the light at the point on the surface, and a `bool` indicating
    /// where the surface is in comparison to the light: `true` if the surface is in front of the
    /// light, and `false` if it's behind and not illuminated.
    fn get_direction(&self, surface: Vertex) -> (Vector, bool);

    /// Get the intensity of the light in the direction of the surface.
    fn get_intensity(&self, surface: Vertex) -> Colour;
}

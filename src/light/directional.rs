//! [`DirectionalLight`], a [`Light`] with constant value in a given direction and no position.
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

use super::Light;

/// `DirectionalLight` is an implementation of [`Light`] with constant value in a given direction.
/// The light has no position and can be treated as infinitely far away.
#[derive(Clone, Debug, PartialEq)]
pub struct DirectionalLight {
    pub direction: Vector,
    pub intensity: Colour,
}

impl Default for DirectionalLight {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self {
            direction: Vector::default(),
            intensity: Colour::default(),
        }
    }
}

impl DirectionalLight {
    /// This is the equivalent of the two-argument constructor from the C++ version.
    pub fn new(direction: Vector, intensity: Colour) -> Self {
        Self {
            direction: direction.normalised(),
            intensity,
        }
    }
}

impl Light for DirectionalLight {
    fn get_direction(&self, _surface: Vertex) -> (Vector, bool) {
        (-self.direction, true)
    }

    fn get_intensity(&self, _surface: Vertex) -> Colour {
        self.intensity
    }
}

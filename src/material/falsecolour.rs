//! [`FalseColour`], a [`Material`] that displays the normal of the surface using RGB values.
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

/// A [`Material`] implementation that maps the x,y,z components of the normal to the r,g,b
/// components of the returned colour. A useful debug tool. -1 to +1 maps to 0 to 1
#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct FalseColour;

impl Material for FalseColour {
    /// The `compute_once()` method maps the normal to the colour to aid debug.
    fn compute_once(&self, _viewer: &Ray, hit: &Hit, _recurse: usize) -> Colour {
        Colour::from_rgb(
            (hit.normal.x + 1.) * 0.5,
            (hit.normal.y + 1.) * 0.5,
            (-hit.normal.z + 1.) * 0.5,
        )
    }

    /// The `compute_per_light()` method makes no contribution.
    fn compute_per_light(&self, _viewer: Vector, _hit: &Hit, _ldir: Vector) -> Colour {
        Colour::from_rgb(0., 0., 0.)
    }
}

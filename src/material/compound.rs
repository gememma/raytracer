//! [`Compound`], a material that combines multiple other materials on a surface.
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

/// `Compound` is a [`Material`][super::Material] that applies multiple other materials to a
/// surface. It can be used to combine a [`Phong`][super::phong::Phong] and a `Global` on a single
/// surface (Rust translation note: what is a `Global`? That doesn't exist. Is it a
/// [`FalseColour`][super::falsecolour::FalseColour]?).
#[derive(Debug)]
pub struct Compound {
    materials: Vec<Box<dyn Material>>,
}

impl Default for Compound {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self {
            materials: Vec::default(),
        }
    }
}

impl Compound {
    /// This is the equivalent of the one-argument constructor from the C++ version.
    ///
    /// I've included this for parity with the C++ version, but this probably won't be that useful;
    /// there's no need to use a fixed-size array for this in Rust (only drawbacks, no benefits),
    /// and so all this does is preallocate `number` elements in the dynamically-sized `Vec` that
    /// is used to store the materials, which is a micro-optimisation.
    pub fn new(number: usize) -> Self {
        Self {
            materials: Vec::with_capacity(number),
        }
    }

    pub fn include_material<M: Material + 'static>(&mut self, material: M) {
        self.materials.push(Box::new(material));
    }
}

impl Material for Compound {
    fn compute_once(&self, viewer: &Ray, hit: &Hit, recurse: usize) -> Colour {
        let mut result = Colour::from_rgb(0., 0., 0.);

        for material in &self.materials {
            result += material.compute_once(viewer, hit, recurse);
        }

        result
    }

    fn compute_per_light(&self, viewer: Vector, hit: &Hit, ldir: Vector) -> Colour {
        let mut result = Colour::default();

        for material in &self.materials {
            result += material.compute_per_light(viewer, hit, ldir);
        }

        result
    }
}

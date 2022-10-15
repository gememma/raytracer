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
pub struct Phong {
    ambient: Colour,
    diffuse: Colour,
    specular: Colour,
    power: f32,
}

impl Phong {
    /// This is the equivalent of the four-argument constructor from the C++ version.
    pub fn new(ambient: Colour, diffuse: Colour, specular: Colour, power: f32) -> Self {
        Phong {
            ambient,
            diffuse,
            specular,
            power,
        }
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
        let ambient_intensity = 0.3;
        self.ambient * ambient_intensity
    }

    /// The `compute_per_light()` method supplies the diffuse and specular terms.
    fn compute_per_light(&self, viewer: Vector, hit: &Hit, ldir: Vector) -> Colour {
        let dotprod = hit.normal.dot(ldir);
        let diffuse = if dotprod < 0. {
            Colour::default()
        } else {
            self.diffuse * dotprod
        };
        let specular = self.specular * (hit.normal.reflection(-ldir).dot(-viewer).powf(self.power));

        // dbg!(diffuse + specular)
        diffuse + specular
    }
}

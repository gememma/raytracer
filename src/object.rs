//! The [`Object`] trait, and [`Plane`][plane::Plane], [`PolyMesh`][polymesh::PolyMesh] and
//! [`Sphere`][sphere::Sphere] implementations.
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

use crate::{hit::Hit, material::Material, ray::Ray, transform::Transform};

use std::fmt::Debug;

pub mod plane;
pub mod polymesh;
pub mod sphere;

/// `Object` is the trait that all objects must conform to.
///
/// ## Rust
///
/// Unlike in C++, `Object` does not also form a linked list of objects (this is for several
/// reasons, not the least of which being that linked lists are difficult to represent in safe Rust
/// code). Instead, you should use a [`Vec`] --- most likely a [`Vec<Box<dyn Object>>`] if you need
/// to store a heterogeneous list of objects.
pub trait Object: Debug {
    /// Get the material this object uses.
    fn material(&self) -> &dyn Material;

    /// Set the material this object uses.
    fn set_material(&mut self, material: Box<dyn Material>);

    /// Given a [`Ray`], if this object intersects it, return a [`Vec`] containing all the points
    /// of intersection. Return an empty [`Vec`] if there are no intersections.
    fn intersection(&self, ray: &Ray) -> Vec<Hit>;

    /// Apply a transform to this object.
    fn apply_transform(&mut self, transform: &Transform);
}

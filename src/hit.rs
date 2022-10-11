//! [`Hit`] represents an intersection between a [`Ray`][crate::ray::Ray] and an [`Object`].
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

use std::fmt;

use crate::{object::Object, vector::Vector, vertex::Vertex};

/// Hit is a struct to store and manipulate information about an intersection between a ray and an
/// object.
///
/// ## Rust
///
/// Unlike in C++, `Hit` does not also form a linked list of hits (this is for several reasons, not
/// the least of which being that linked lists are difficult to represent in safe Rust code).
/// Instead, you should use a [`Vec<Hit>`].
///
/// `Hit` also does not implement the arena-allocation feature from the C++ version.
#[derive(Clone, Debug)]
pub struct Hit<'obj> {
    /// The intersection distance.
    pub t: f32,
    /// `true` if entering an object, `false` if leaving.
    pub entering: bool,
    /// The object that was hit.
    pub what: &'obj dyn Object,
    /// The position of intersection.
    pub position: Vertex,
    /// The normal at the point of intersection.
    pub normal: Vector,
}

/// This is the equivalent of the `operator<<` implementation from the C++ version. To use it,
/// print a hit using the `{}` format specifier, e.g.:
///
/// ```
/// # use raytracer::{
/// #     hit::Hit,
/// #     object::{Object, sphere::Sphere},
/// #     vertex::Vertex,
/// #     ray::Ray,
/// #     vector::Vector
/// # };
/// let object = Sphere::new(Vertex::from_xyz(0., 0., 2.), 1.);
/// let ray = Ray::new(Vertex::from_xyz(0., 0., 0.), Vector::new(0., 0., 1.));
/// let hit = &object.intersection(&ray)[0];
/// println!("{}", hit);
/// # let mut printed = String::new();
/// # use std::fmt::Write;
/// # write!(printed, "{}", hit);
/// # assert_eq!(printed, "Hit{,[0,0,1],[0,0,-1]}");
/// // prints: Hit{,[0,0,1],[0,0,-1]}
/// ```
impl fmt::Display for Hit<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            // The double brackets in {{ and }} are intentional: because Rust uses {} to represent
            // an interpolated value in a format string, {{ means "print a single { character".
            "Hit{{,[{},{},{}],[{},{},{}]}}",
            self.position.x,
            self.position.y,
            self.position.z,
            self.normal.x,
            self.normal.y,
            self.normal.z
        )
    }
}

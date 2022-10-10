//! [`Ray`], a struct to store and manipulate 3D rays.
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

use crate::{vector::Vector, vertex::Vertex};

/// Ray is a struct to store and manipulate 3D rays.
#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    pub position: Vertex,
    pub direction: Vector,
}

impl Default for Ray {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self {
            position: Vertex::default(),
            direction: Vector::default(),
        }
    }
}

impl Ray {
    /// This is the equivalent of the two-argument constructor from the C++ version.
    pub fn new(position: Vertex, direction: Vector) -> Self {
        Self {
            position,
            direction,
        }
    }
}

/// This is the equivalent of the `operator<<` implementation from the C++ version. To use it,
/// print a ray using the `{}` format specifier, e.g.:
///
/// ```
/// # use krt::{
/// #     ray::Ray,
/// #     vertex::Vertex,
/// #     vector::Vector
/// # };
/// let ray = Ray::new(Vertex::from_xyz(0., 0., 0.), Vector::new(0., 0., 1.));
/// println!("{}", ray);
/// # let mut printed = String::new();
/// # use std::fmt::Write;
/// # write!(printed, "{}", ray);
/// # assert_eq!(printed, "Ray{[0,0,0],[0,0,1]}");
/// // prints: Ray{[0,0,0],[0,0,1]}
/// ```
impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Ray{{[{},{},{}],[{},{},{}]}}",
            self.position.x,
            self.position.y,
            self.position.z,
            self.direction.x,
            self.direction.y,
            self.direction.z
        )
    }
}

//! [`Vertex`], a four element vector struct with lots of operators and common functions.
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

use std::ops::{Add, Neg, Sub};

use crate::vector::Vector;

/// A four element vector struct with lots of operators and common functions.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Vertex {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
            w: 1.,
        }
    }
}

impl Vertex {
    /// This is the equivalent of the four-argument constructor from the C++ version. Rust does
    /// not allow overloading, so the constructors must have different names.
    pub fn from_xyzw(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// This is the equivalent of the three-argument constructor from the C++ version. Rust does
    /// not allow overloading, so the constructors must have different names.
    pub fn from_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1. }
    }
}

impl Add<Vector> for Vertex {
    type Output = Vertex;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::from_xyzw(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w)
    }
}

impl Sub<Vector> for Vertex {
    type Output = Vertex;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::from_xyzw(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z, self.w)
    }
}

impl Neg for Vertex {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::from_xyzw(-self.x, -self.y, -self.z, -self.w)
    }
}

// Unlike in the C++ implementation, the two types Vertex and Vector are not actually related. This
// means that some additional implementations of operators and mathematical functions are needed
// for Vertex that are provided implicitly by subclassing in the C++ version. It's possible I've
// missed translating some of them that are needed: sorry! This is alphabetically the last file and
// I can't be bothered. I'm sure you'll be able to implement them as needed. Good luck on your
// coursework, and please don't forget to thank Ken for letting you use this starter code instead
// of the C++ version! He was under no obligation to provide this to you.

impl Vertex {
    pub fn normalise(&mut self) {
        self.x /= self.length();
        self.y /= self.length();
        self.z /= self.length();
    }

    pub fn normalised(&self) -> Self {
        let mut normalised = self.clone();
        normalised.normalise();
        normalised
    }

    pub fn len_sqr(&self) -> f32 {
        self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)
    }

    pub fn length(&self) -> f32 {
        (self.len_sqr() as f64).sqrt() as f32
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
}

impl Sub<Vertex> for Vertex {
    type Output = Self;

    fn sub(self, rhs: Vertex) -> Self::Output {
        Self::from_xyzw(
            self.x - rhs.x,
            self.y - rhs.y,
            self.z - rhs.z,
            self.w - rhs.w,
        )
    }
}

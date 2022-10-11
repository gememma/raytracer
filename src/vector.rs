//! [`Vector`], a three element vector struct with lots of operators and common functions.
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

use std::ops::{Add, Mul, Neg, Sub};

use crate::vertex::Vertex;

/// A three element vector struct with lots of operators and common functions.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector {
    /// This is the equivalent of the three-argument constructor from the C++ version.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

impl Default for Vector {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}

impl Vector {
    pub fn normalise(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
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

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn reflection(&self, initial: Self) -> Self {
        let d = self.dot(initial) * 2.;

        Self::new(
            initial.x - d * self.x,
            initial.y - d * self.y,
            initial.z - d * self.z,
        )
    }

    pub fn negate(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }

    /// This is the equivalent of the `cross(Vector&, Vector&)` method from C++. It doesn't use
    /// out-parameters because that's a C++ idiom that should stay in C++.
    pub fn cross(&self, other: Self) -> Self {
        Self::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }

    /// This is the equivalent of the `cross(Vector&)` method from C++, but it has to be called
    /// something else because Rust doesn't allow function overloading.
    pub fn cross_in_place(&mut self, other: Self) {
        let x = self.y * other.z - self.z * other.y;
        let y = self.z * other.x - self.x * other.z;
        let z = self.x * other.y - self.y * other.x;

        self.x = x;
        self.y = y;
        self.z = z;
    }
}

impl Mul<Vector> for Vector {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl Sub<Vector> for Vector {
    type Output = Self;

    fn sub(self, rhs: Vector) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Mul<f32> for Vector {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Self::Output {
        Vector::new(self * rhs.x, self * rhs.y, self * rhs.z)
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

/// Convert a [`Vertex`] to a [`Vector`], normalising the vertex first.
impl From<Vertex> for Vector {
    fn from(mut vertex: Vertex) -> Self {
        vertex.normalise();

        Self {
            x: vertex.x,
            y: vertex.y,
            z: vertex.z,
        }
    }
}

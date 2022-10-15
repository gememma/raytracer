//! [`Colour`], an RGBA colour struct.
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

use std::ops::{Add, AddAssign, Mul, MulAssign};

/// Colour is a struct to store and maniplulate an RGBA colour.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Colour {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.,
        }
    }
}

impl Colour {
    /// This is the equivalent of the three-argument constructor from the C++ version. Rust does
    /// not allow overloading, so the constructors must have different names.
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b,
            ..Default::default()
        }
    }

    /// This is the equivalent of the four-argument constructor from the C++ version. Rust does not
    /// allow overloading, so the constructors must have different names.
    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn scale(&mut self, scaling: Colour) {
        self.r *= scaling.r;
        self.g *= scaling.g;
        self.b *= scaling.b;
        self.a *= scaling.a;
    }

    pub fn add(&mut self, adjust: Colour) {
        self.r += adjust.r;
        self.g += adjust.g;
        self.b += adjust.b;
    }
}

/// This is the equivalent of the `operator*` implementation for `Colour&` from the C++ version.
impl Mul<&Colour> for Colour {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self {
        Self::from_rgba(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b,
            self.a * rhs.a,
        )
    }
}

/// This is the equivalent of the `operator*` implementation for `Colour` from the C++ version.
impl Mul<Colour> for Colour {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::from_rgba(
            self.r * rhs.r,
            self.g * rhs.g,
            self.b * rhs.b,
            self.a * rhs.a,
        )
    }
}

/// This is the equivalent of the `operator+` implementation for `Colour&` from the C++ version.
impl Add<&Colour> for Colour {
    type Output = Self;

    fn add(self, rhs: &Self) -> Self::Output {
        Self::from_rgba(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            (self.a + rhs.a).clamp(0., 1.),
        )
    }
}

/// This is the equivalent of the `operator+` implementation for `Colour` from the C++ version.
impl Add<Colour> for Colour {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_rgba(
            self.r + rhs.r,
            self.g + rhs.g,
            self.b + rhs.b,
            self.a + rhs.a,
        )
    }
}

/// This is the equivalent of the `operator*` implementation for `float` from the C++ version.
impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_rgba(self.r * rhs, self.g * rhs, self.b * rhs, self.a)
    }
}

/// This is the equivalent of the `operator+=` implementation for `Colour&` from the C++ version.
impl AddAssign<&Colour> for Colour {
    fn add_assign(&mut self, rhs: &Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

/// This is the equivalent of the `operator+=` implementation for `Colour` from the C++ version.
impl AddAssign<Colour> for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

/// This is the equivalent of the `operator*=` implementation for `Colour&` from the C++ version.
impl MulAssign<&Colour> for Colour {
    fn mul_assign(&mut self, rhs: &Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

/// This is the equivalent of the `operator*=` implementation for `Colour` from the C++ version.
impl MulAssign<Colour> for Colour {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

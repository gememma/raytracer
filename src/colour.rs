use rand::Rng;
use std::ops::{Add, AddAssign, Mul, MulAssign};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Colour {
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
    pub fn from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            r,
            g,
            b,
            ..Default::default()
        }
    }

    pub fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn random(min: f32, max: f32) -> Self {
        let r = rand::thread_rng().gen_range(min..max);
        let g = rand::thread_rng().gen_range(min..max);
        let b = rand::thread_rng().gen_range(min..max);
        Self::from_rgb(r, g, b)
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

impl Mul<f32> for Colour {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::from_rgba(self.r * rhs, self.g * rhs, self.b * rhs, self.a)
    }
}

impl AddAssign<&Colour> for Colour {
    fn add_assign(&mut self, rhs: &Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl AddAssign<Colour> for Colour {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
        self.a += rhs.a;
    }
}

impl MulAssign<&Colour> for Colour {
    fn mul_assign(&mut self, rhs: &Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

impl MulAssign<Colour> for Colour {
    fn mul_assign(&mut self, rhs: Self) {
        self.r *= rhs.r;
        self.g *= rhs.g;
        self.b *= rhs.b;
        self.a *= rhs.a;
    }
}

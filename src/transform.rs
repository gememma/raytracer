//! [`Transform`], a struct to store, manipulate and apply transforms.
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

use std::ops::Mul;

use crate::{vector::Vector, vertex::Vertex};

/// Transform is a struct to store, manipulate and apply transforms.
#[derive(Clone, Debug, PartialEq)]
pub struct Transform {
    pub matrix: [[f32; 4]; 4],
}

impl Default for Transform {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self::new(
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.],
        )
    }
}

impl Transform {
    /// This is the equivalent of the sixteen-argument constructor from the C++ version.
    pub fn new(
        [a, b, c, d]: [f32; 4],
        [e, f, g, h]: [f32; 4],
        [i, j, k, l]: [f32; 4],
        [m, n, o, p]: [f32; 4],
    ) -> Self {
        Self {
            matrix: [[a, b, c, d], [e, f, g, h], [i, j, k, l], [m, n, o, p]],
        }
    }
}

/// Trait to allow for multiple implementations of the same methods with different types, because
/// Rust does not otherwise allow for function overloading.
pub trait Apply<T> {
    fn apply(&self, other: T) -> T;
    fn apply_to(&self, other: &mut T);
}

impl Apply<Vertex> for Transform {
    fn apply(&self, input: Vertex) -> Vertex {
        Vertex {
            x: self.matrix[0][0] * input.x
                + self.matrix[0][1] * input.y
                + self.matrix[0][2] * input.z
                + self.matrix[0][3] * input.w,
            y: self.matrix[1][0] * input.x
                + self.matrix[1][1] * input.y
                + self.matrix[1][2] * input.z
                + self.matrix[1][3] * input.w,
            z: self.matrix[2][0] * input.x
                + self.matrix[2][1] * input.y
                + self.matrix[2][2] * input.z
                + self.matrix[2][3] * input.w,
            w: self.matrix[3][0] * input.x
                + self.matrix[3][1] * input.y
                + self.matrix[3][2] * input.z
                + self.matrix[3][3] * input.w,
        }
    }

    fn apply_to(&self, vertex: &mut Vertex) {
        let x = self.matrix[0][0] * vertex.x
            + self.matrix[0][1] * vertex.y
            + self.matrix[0][2] * vertex.z
            + self.matrix[0][3] * vertex.w;
        let y = self.matrix[1][0] * vertex.x
            + self.matrix[1][1] * vertex.y
            + self.matrix[1][2] * vertex.z
            + self.matrix[1][3] * vertex.w;
        let z = self.matrix[2][0] * vertex.x
            + self.matrix[2][1] * vertex.y
            + self.matrix[2][2] * vertex.z
            + self.matrix[2][3] * vertex.w;
        let w = self.matrix[3][0] * vertex.x
            + self.matrix[3][1] * vertex.y
            + self.matrix[3][2] * vertex.z
            + self.matrix[3][3] * vertex.w;

        vertex.x = x;
        vertex.y = y;
        vertex.z = z;
        vertex.w = w;
    }
}

impl Apply<Vector> for Transform {
    fn apply(&self, input: Vector) -> Vector {
        Vector {
            x: self.matrix[0][0] * input.x
                + self.matrix[0][1] * input.y
                + self.matrix[0][2] * input.z,
            y: self.matrix[1][0] * input.x
                + self.matrix[1][1] * input.y
                + self.matrix[1][2] * input.z,
            z: self.matrix[2][0] * input.x
                + self.matrix[2][1] * input.y
                + self.matrix[2][2] * input.z,
        }
    }

    fn apply_to(&self, vector: &mut Vector) {
        let x = self.matrix[0][0] * vector.x
            + self.matrix[0][1] * vector.y
            + self.matrix[0][2] * vector.z;
        let y = self.matrix[1][0] * vector.x
            + self.matrix[1][1] * vector.y
            + self.matrix[1][2] * vector.z;
        let z = self.matrix[2][0] * vector.x
            + self.matrix[2][1] * vector.y
            + self.matrix[2][2] * vector.z;

        vector.x = x;
        vector.y = y;
        vector.z = z;
    }
}

/// This is the equivalent of the `operator*` implementation for `Transform&` from the C++ version.
impl Mul<&Transform> for Transform {
    type Output = Self;

    #[allow(clippy::needless_range_loop)]
    fn mul(self, rhs: &Self) -> Self::Output {
        let mut matrix = [[0.; 4]; 4];

        for x in 0..4 {
            for y in 0..4 {
                matrix[x][y] = self.matrix[x][0] * rhs.matrix[0][y]
                    + self.matrix[x][1] * rhs.matrix[1][y]
                    + self.matrix[x][2] * rhs.matrix[2][y]
                    + self.matrix[x][3] * rhs.matrix[3][y];
            }
        }

        Self { matrix }
    }
}

impl Mul<Transform> for Transform {
    type Output = Self;

    #[allow(clippy::needless_range_loop)]
    fn mul(self, rhs: Self) -> Self::Output {
        let mut matrix = [[0.; 4]; 4];

        for x in 0..4 {
            for y in 0..4 {
                matrix[x][y] = self.matrix[x][0] * rhs.matrix[0][y]
                    + self.matrix[x][1] * rhs.matrix[1][y]
                    + self.matrix[x][2] * rhs.matrix[2][y]
                    + self.matrix[x][3] * rhs.matrix[3][y];
            }
        }

        Self { matrix }
    }
}

impl Transform {
    pub fn inverse(&self) -> Self {
        let mut matrix = [[0.; 4]; 4];

        matrix[0][0] = self.matrix[1][1] * self.matrix[2][2] * self.matrix[3][3]
            - self.matrix[1][1] * self.matrix[2][3] * self.matrix[3][2]
            - self.matrix[2][1] * self.matrix[1][2] * self.matrix[3][3]
            + self.matrix[2][1] * self.matrix[1][3] * self.matrix[3][2]
            + self.matrix[3][1] * self.matrix[1][2] * self.matrix[2][3]
            - self.matrix[3][1] * self.matrix[1][3] * self.matrix[2][2];

        matrix[1][0] = -self.matrix[1][0] * self.matrix[2][2] * self.matrix[3][3]
            + self.matrix[1][0] * self.matrix[2][3] * self.matrix[3][2]
            + self.matrix[2][0] * self.matrix[1][2] * self.matrix[3][3]
            - self.matrix[2][0] * self.matrix[1][3] * self.matrix[3][2]
            - self.matrix[3][0] * self.matrix[1][2] * self.matrix[2][3]
            + self.matrix[3][0] * self.matrix[1][3] * self.matrix[2][2];

        matrix[2][0] = self.matrix[1][0] * self.matrix[2][1] * self.matrix[3][3]
            - self.matrix[1][0] * self.matrix[2][3] * self.matrix[3][1]
            - self.matrix[2][0] * self.matrix[1][1] * self.matrix[3][3]
            + self.matrix[2][0] * self.matrix[1][3] * self.matrix[3][1]
            + self.matrix[3][0] * self.matrix[1][1] * self.matrix[2][3]
            - self.matrix[3][0] * self.matrix[1][3] * self.matrix[2][1];

        matrix[3][0] = -self.matrix[1][0] * self.matrix[2][1] * self.matrix[3][2]
            + self.matrix[1][0] * self.matrix[2][2] * self.matrix[3][1]
            + self.matrix[2][0] * self.matrix[1][1] * self.matrix[3][2]
            - self.matrix[2][0] * self.matrix[1][2] * self.matrix[3][1]
            - self.matrix[3][0] * self.matrix[1][1] * self.matrix[2][2]
            + self.matrix[3][0] * self.matrix[1][2] * self.matrix[2][1];

        matrix[0][1] = -self.matrix[0][1] * self.matrix[2][2] * self.matrix[3][3]
            + self.matrix[0][1] * self.matrix[2][3] * self.matrix[3][2]
            + self.matrix[2][1] * self.matrix[0][2] * self.matrix[3][3]
            - self.matrix[2][1] * self.matrix[0][3] * self.matrix[3][2]
            - self.matrix[3][1] * self.matrix[0][2] * self.matrix[2][3]
            + self.matrix[3][1] * self.matrix[0][3] * self.matrix[2][2];

        matrix[1][1] = self.matrix[0][0] * self.matrix[2][2] * self.matrix[3][3]
            - self.matrix[0][0] * self.matrix[2][3] * self.matrix[3][2]
            - self.matrix[2][0] * self.matrix[0][2] * self.matrix[3][3]
            + self.matrix[2][0] * self.matrix[0][3] * self.matrix[3][2]
            + self.matrix[3][0] * self.matrix[0][2] * self.matrix[2][3]
            - self.matrix[3][0] * self.matrix[0][3] * self.matrix[2][2];

        matrix[2][1] = -self.matrix[0][0] * self.matrix[2][1] * self.matrix[3][3]
            + self.matrix[0][0] * self.matrix[2][3] * self.matrix[3][1]
            + self.matrix[2][0] * self.matrix[0][1] * self.matrix[3][3]
            - self.matrix[2][0] * self.matrix[0][3] * self.matrix[3][1]
            - self.matrix[3][0] * self.matrix[0][1] * self.matrix[2][3]
            + self.matrix[3][0] * self.matrix[0][3] * self.matrix[2][1];

        matrix[3][1] = self.matrix[0][0] * self.matrix[2][1] * self.matrix[3][2]
            - self.matrix[0][0] * self.matrix[2][2] * self.matrix[3][1]
            - self.matrix[2][0] * self.matrix[0][1] * self.matrix[3][2]
            + self.matrix[2][0] * self.matrix[0][2] * self.matrix[3][1]
            + self.matrix[3][0] * self.matrix[0][1] * self.matrix[2][2]
            - self.matrix[3][0] * self.matrix[0][2] * self.matrix[2][1];

        matrix[0][2] = self.matrix[0][1] * self.matrix[1][2] * self.matrix[3][3]
            - self.matrix[0][1] * self.matrix[1][3] * self.matrix[3][2]
            - self.matrix[1][1] * self.matrix[0][2] * self.matrix[3][3]
            + self.matrix[1][1] * self.matrix[0][3] * self.matrix[3][2]
            + self.matrix[3][1] * self.matrix[0][2] * self.matrix[1][3]
            - self.matrix[3][1] * self.matrix[0][3] * self.matrix[1][2];

        matrix[1][2] = -self.matrix[0][0] * self.matrix[1][2] * self.matrix[3][3]
            + self.matrix[0][0] * self.matrix[1][3] * self.matrix[3][2]
            + self.matrix[1][0] * self.matrix[0][2] * self.matrix[3][3]
            - self.matrix[1][0] * self.matrix[0][3] * self.matrix[3][2]
            - self.matrix[3][0] * self.matrix[0][2] * self.matrix[1][3]
            + self.matrix[3][0] * self.matrix[0][3] * self.matrix[1][2];

        matrix[2][2] = self.matrix[0][0] * self.matrix[1][1] * self.matrix[3][3]
            - self.matrix[0][0] * self.matrix[1][3] * self.matrix[3][1]
            - self.matrix[1][0] * self.matrix[0][1] * self.matrix[3][3]
            + self.matrix[1][0] * self.matrix[0][3] * self.matrix[3][1]
            + self.matrix[3][0] * self.matrix[0][1] * self.matrix[1][3]
            - self.matrix[3][0] * self.matrix[0][3] * self.matrix[1][1];

        matrix[3][2] = -self.matrix[0][0] * self.matrix[1][1] * self.matrix[3][2]
            + self.matrix[0][0] * self.matrix[1][2] * self.matrix[3][1]
            + self.matrix[1][0] * self.matrix[0][1] * self.matrix[3][2]
            - self.matrix[1][0] * self.matrix[0][2] * self.matrix[3][1]
            - self.matrix[3][0] * self.matrix[0][1] * self.matrix[1][2]
            + self.matrix[3][0] * self.matrix[0][2] * self.matrix[1][1];

        matrix[0][3] = -self.matrix[0][1] * self.matrix[1][2] * self.matrix[2][3]
            + self.matrix[0][1] * self.matrix[1][3] * self.matrix[2][2]
            + self.matrix[1][1] * self.matrix[0][2] * self.matrix[2][3]
            - self.matrix[1][1] * self.matrix[0][3] * self.matrix[2][2]
            - self.matrix[2][1] * self.matrix[0][2] * self.matrix[1][3]
            + self.matrix[2][1] * self.matrix[0][3] * self.matrix[1][2];

        matrix[1][3] = self.matrix[0][0] * self.matrix[1][2] * self.matrix[2][3]
            - self.matrix[0][0] * self.matrix[1][3] * self.matrix[2][2]
            - self.matrix[1][0] * self.matrix[0][2] * self.matrix[2][3]
            + self.matrix[1][0] * self.matrix[0][3] * self.matrix[2][2]
            + self.matrix[2][0] * self.matrix[0][2] * self.matrix[1][3]
            - self.matrix[2][0] * self.matrix[0][3] * self.matrix[1][2];

        matrix[2][3] = -self.matrix[0][0] * self.matrix[1][1] * self.matrix[2][3]
            + self.matrix[0][0] * self.matrix[1][3] * self.matrix[2][1]
            + self.matrix[1][0] * self.matrix[0][1] * self.matrix[2][3]
            - self.matrix[1][0] * self.matrix[0][3] * self.matrix[2][1]
            - self.matrix[2][0] * self.matrix[0][1] * self.matrix[1][3]
            + self.matrix[2][0] * self.matrix[0][3] * self.matrix[1][1];

        matrix[3][3] = self.matrix[0][0] * self.matrix[1][1] * self.matrix[2][2]
            - self.matrix[0][0] * self.matrix[1][2] * self.matrix[2][1]
            - self.matrix[1][0] * self.matrix[0][1] * self.matrix[2][2]
            + self.matrix[1][0] * self.matrix[0][2] * self.matrix[2][1]
            + self.matrix[2][0] * self.matrix[0][1] * self.matrix[1][2]
            - self.matrix[2][0] * self.matrix[0][2] * self.matrix[1][1];

        let det = self.matrix[0][0] * matrix[0][0]
            + self.matrix[0][1] * matrix[1][0]
            + self.matrix[0][2] * matrix[2][0]
            + self.matrix[0][3] * matrix[3][0];

        if det == 0. {
            Self { matrix }
        } else {
            let det = 1. / det;

            for element in matrix.iter_mut().flatten() {
                *element *= det;
            }

            Self { matrix }
        }
    }

    #[allow(clippy::needless_range_loop)]
    pub fn transpose(&self) -> Self {
        let mut matrix = [[0.; 4]; 4];

        for x in 0..4 {
            for y in 0..4 {
                matrix[x][y] = self.matrix[y][x];
            }
        }

        Self { matrix }
    }
}

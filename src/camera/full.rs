//! [`FullCamera`], a [`Camera`] that can be placed at a position in space with a look-at and up
//! direction.
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

use crate::{
    environment::Environment, framebuffer::FrameBuffer, ray::Ray, vector::Vector, vertex::Vertex,
};

use super::Camera;

/// `FullCamera` allows a camera to be placed in space with a lookat and up direction as well as
/// the field of view. It loops over the pixels in a [`FrameBuffer`] and computes a ray that is
/// then passed to the environment.
#[derive(Clone, Debug, PartialEq)]
pub struct FullCamera {
    pub width: usize,
    pub height: usize,
    pub fov: f32,
    pub position: Vertex,
    pub lookat: Vector,
    pub up: Vector,
    pub right: Vector,
}

impl Default for FullCamera {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        todo!("you must implement the default constructor for FullCamera")
    }
}

impl FullCamera {
    /// This is the equivalent of the four-argument constructor from the C++ version.
    pub fn new(_fov: f32, _position: Vertex, _lookat: Vector, _up: Vector) -> Self {
        todo!("you must implement the parameterised constructor for FullCamera")
    }

    pub fn get_ray_offset(_x: usize, _y: usize, _ox: f32, _oy: f32) -> Ray {
        todo!("you must implement getting a ray with offsets for FullCamera")
    }

    pub fn get_ray_pixel(_x: usize, _y: usize) -> Ray {
        todo!("you must implement getting a ray for a pixel for FullCamera")
    }
}

impl Camera for FullCamera {
    fn render<E: Environment>(&self, _env: &E, _fb: &mut FrameBuffer) {
        todo!("you must implement rendering an environment with a FullCamera")
    }
}

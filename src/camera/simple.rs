//! [`SimpleCamera`], a [`Camera`] with a 90 degree field of view along the z axis.
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

use crate::scene::Scene;
use crate::{framebuffer::FrameBuffer, ray::Ray, Vertex};
use glam::Vec3A;

use super::Camera;

/// `SimpleCamera` has a 90 degree field of view along the z axis.
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleCamera {
    pub width: f32,
    pub height: f32,
    pub fov: f32,
}

impl Default for SimpleCamera {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self {
            width: 512.,
            height: 512.,
            fov: 0.5,
        }
    }
}

impl SimpleCamera {
    /// This is the equivalent of the one-argument constructor from the C++ version.
    pub fn with_fov(fov: f32) -> Self {
        Self {
            fov,
            ..Default::default()
        }
    }

    pub fn get_ray_pixel(&self, x: usize, y: usize) -> Ray {
        let fx = (x as f32 + 0.5) / self.width;
        let fy = (y as f32 + 0.5) / self.height;

        Ray::new(
            Vertex::new(0., 0., 0.),
            Vec3A::new(fx - 0.5, 0.5 - fy, self.fov).normalize(),
        )
    }
}

impl Camera for SimpleCamera {
    fn render(&self, env: Scene, fb: &mut FrameBuffer) {
        for y in 0..fb.height() {
            for x in 0..fb.width() {
                let ray = self.get_ray_pixel(x, y);

                let (colour, depth) = env.raytrace(ray, 5, Vertex::default());

                fb.plot_pixel(x, y, colour.r, colour.g, colour.b);
                fb.plot_depth(x, y, depth);
            }
        }
    }
}

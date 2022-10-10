//! The [`Camera`] trait, and [`SimpleCamera`][simple::SimpleCamera] and
//! [`FullCamera`][full::FullCamera] implementations.
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

use crate::{environment::Environment, framebuffer::FrameBuffer};

pub mod full;
pub mod simple;

/// Camera is the trait that all cameras must implement.
pub trait Camera: Default {
    /// Given an [`Environment`] (typically a [`Scene`][crate::scene::Scene]), fill in the
    /// framebuffer pixels, both colour and depth.
    fn render<E: Environment>(&self, env: &E, fb: &mut FrameBuffer);
}

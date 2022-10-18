//! [`Sphere`], a simple sphere [`Object`].
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
    hit::Hit,
    material::{falsecolour::FalseColour, Material},
    ray::Ray,
    Vertex,
};
use glam::{Affine3A, Vec3A};

use super::Object;

/// A simple sphere object.
#[derive(Debug)]
pub struct Sphere {
    center: Vertex,
    radius: f32,
    material: Box<dyn Material>,
}

impl Sphere {
    /// This is the equivalent of the two-argument constructor from the C++ version.
    pub fn new(center: Vertex, radius: f32) -> Self {
        Self {
            center,
            radius,
            material: Box::new(FalseColour::default()),
        }
    }
}

impl Object for Sphere {
    fn material(&self) -> &dyn Material {
        &*self.material
    }

    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        // offset ray by sphere position
        // equivalent to transforming ray into local sphere space
        let ro = Vec3A::new(
            ray.position.x - self.center.x,
            ray.position.y - self.center.y,
            ray.position.z - self.center.z,
        );

        let a = ray.direction.dot(ray.direction);
        let b = 2. * ray.direction.dot(ro);
        let c = ro.dot(ro) - self.radius.powi(2);

        let disc = b.powi(2) - 4. * a * c;

        if disc < 0. {
            Vec::new()
        } else {
            let ds = disc.sqrt();

            let t0 = (-b - ds) / 2.;
            let t1 = (-b + ds) / 2.;

            let position0 = ray.position + t0 * ray.direction;
            let mut hit0 = Hit {
                t: t0,
                entering: true,
                object_hit: self,
                position: position0,
                normal: (position0 - self.center).normalize(),
            };

            if hit0.normal.dot(ray.direction) > 0. {
                hit0.normal *= -1.;
            }

            let position1 = ray.position + t1 * ray.direction;
            let mut hit1 = Hit {
                t: t1,
                entering: false,
                object_hit: self,
                position: position1,
                normal: (position1 - self.center).normalize(),
            };

            if hit1.normal.dot(ray.direction) > 0. {
                hit1.normal *= -1.;
            }

            vec![hit0, hit1]
        }
    }

    fn apply_transform(&mut self, t: Affine3A) {
        let new = t.transform_point3a(self.center);
        self.center = new;
    }
}

//! [`Scene`], a struct that is used to build a scene database of [`Object`]s and [`Light`]s and
//! then trace a [`Ray`] through it.
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
    colour::Colour, environment::Environment, hit::Hit, light::Light, object::Object, ray::Ray,
};

/// `Scene` is a struct that is used to build a scene database of [`Object`]s and [`Light`]s and
/// then trace a [`Ray`] through it.
#[derive(Debug)]
pub struct Scene {
    pub object_list: Vec<Box<dyn Object>>,
    pub light_list: Vec<Box<dyn Light>>,
}

impl Default for Scene {
    /// This is the equivalent of the default (no-argument) constructor from the C++ version.
    fn default() -> Self {
        Self {
            object_list: Vec::default(),
            light_list: Vec::default(),
        }
    }
}

impl Scene {
    /// Trace a [`Ray`] through the scene and find the closest if any object intersection in front
    /// of the ray.
    pub fn trace(&self, ray: &Ray) -> Option<Hit> {
        let mut best_hit = None;

        for object in &self.object_list {
            if let Some(new_hit) = Self::select_first(object.intersection(ray)) {
                match best_hit {
                    None => {
                        best_hit = Some(new_hit);
                    }
                    Some(old_best) if new_hit.t < old_best.t => {
                        best_hit = Some(new_hit);
                    }
                    _ => {}
                }
            }
        }

        best_hit
    }

    /// Filter the list of returned hits to the closest +ve.
    pub fn select_first(hits: Vec<Hit>) -> Option<Hit> {
        let mut result: Option<Hit> = None;

        for hit in hits {
            // if hit.t >= 0. {
            //     result = Some(hit);
            //     break;
            // }
            if let Some(h) = &result {
                if hit.t < h.t && hit.t >= 0. {
                    result = Some(hit);
                }
            } else if hit.t >= 0. {
                result = Some(hit);
            }
        }
        result
    }
}

impl Environment for Scene {
    /// Raytrace a shadow ray.
    fn shadowtrace(&self, ray: &Ray, limit: f32) -> bool {
        for object in &self.object_list {
            if let Some(hit) = Self::select_first(object.intersection(ray)) {
                if hit.t > 0.00000001 && hit.t < limit {
                    return true;
                }
            }
        }

        return false;
    }

    /// Trace a [`Ray`] through the scene and return its [`Colour`]. This function is the one that
    /// should recurse down the reflection/refraction tree within a material.
    fn raytrace(&self, ray: Ray, recurse: usize) -> (Colour, f32) {
        // a default colour if we hit nothing.
        let mut colour = Colour::from_rgba(0., 0., 0., 0.);
        let mut depth = 0.;

        // first step, find the closest primitive
        let best_hit = self.trace(&ray);

        // if we found a primitive then compute the colour we should see
        if let Some(best) = best_hit {
            depth = best.t;
            colour += best.what.material().compute_once(&ray, &best, recurse);

            // next, compute the light contribution for each light in the scene.
            for light in &self.light_list {
                let viewer = -best.position.normalised();

                let (ldir, mut lit) = light.get_direction(best.position);

                if ldir.dot(best.normal) > 0. {
                    // Light is facing wrong way.
                    lit = false;
                }

                // TODO: Put the shadow check here, if lit==true and in shadow, set lit==false

                if lit {
                    let intensity = light.get_intensity(best.position);
                    colour += intensity
                        * best
                            .what
                            .material()
                            .compute_per_light(viewer.into(), &best, ldir);
                }
            }
        } else {
            colour.r = 0.;
            colour.g = 0.;
            colour.b = 0.;
            colour.a = 1.;
        }

        (colour, depth)
    }
}

impl Scene {
    pub fn add_object<O: Object + 'static>(&mut self, object: O) {
        self.object_list.push(Box::new(object));
    }

    pub fn add_light<L: Light + 'static>(&mut self, light: L) {
        self.light_list.push(Box::new(light));
    }
}

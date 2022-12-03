use std::{sync::mpsc::channel, thread};

use glam::Vec3A;
use rand::Rng;

use crate::{
    colour::Colour, framebuffer::FrameBuffer, photonmap::PhotonMap, ray::Ray, scene::Scene, Vertex,
};

#[derive(Clone, Debug, PartialEq)]
pub struct FullCamera {
    pub width: usize,
    pub height: usize,
    pub fov: f32,
    pub position: Vertex,
    pub w: Vec3A,
    pub v: Vec3A,
    pub u: Vec3A,
    bottom_left_pixel: Vec3A,
    samples: usize,
    lens_radius: f32,
    focal_distance: f32,
}

impl FullCamera {
    pub fn new(
        fov: f32,
        position: Vertex,
        look: Vertex,
        up: Vec3A,
        width: usize,
        height: usize,
        samples: usize,
        aperture: f32,
    ) -> Self {
        let w = (look - position).normalize();
        let u = (up.cross(w)).normalize();
        let v = w.cross(u);

        let focal_distance = (look - position).length();
        let horizontal = 0.5 * u;
        let vertical = 0.5 * v;

        let bottom_left_pixel = position + (-horizontal - vertical + fov * w) * focal_distance;
        let lens_radius = aperture / 2.; // used to create sample rays, this is ok

        Self {
            width,
            height,
            fov,
            position,
            w,
            v,
            u,
            bottom_left_pixel,
            samples,
            lens_radius,
            focal_distance,
        }
    }

    pub fn get_ray_pixel(&self, x: usize, y: usize) -> Ray {
        // convert pixel coordinates to world coordinates
        // add a small amount of randomness
        let mut rng = rand::thread_rng();
        // offset simulates thin lens model
        let offset = random_in_unit_disc() * self.lens_radius;
        Ray::new(
            self.position + offset,
            (self.bottom_left_pixel
                + (x as f32 + rng.gen::<f32>()) / self.width as f32 * self.u * self.focal_distance
                + (y as f32 + rng.gen::<f32>()) / self.height as f32
                    * self.v
                    * self.focal_distance
                - self.position
                - offset)
                .normalize(),
        )
    }
    pub fn render(&self, env: &Scene, fb: &mut FrameBuffer) {
        // This method spawns threads that raytrace in parallel for speed
        thread::scope(|s| {
            let (tx, rx) = channel();
            let thread_num = 8;
            for i in 0..thread_num {
                let tx = tx.clone();
                let starty = i * fb.height() / thread_num;
                let endy = starty + fb.height() / thread_num;

                let width = fb.width();

                s.spawn(move || {
                    for y in starty..endy {
                        for x in 0..width {
                            let mut colour = Colour::default();
                            let mut depth = 0.;
                            for _ in 0..self.samples {
                                let ray = self.get_ray_pixel(x, y);
                                let (colourtmp, depthtmp) = env.raytrace(ray, 5, self.position);
                                colour += colourtmp / self.samples as f32;
                                depth += depthtmp / self.samples as f32;
                            }
                            tx.send((colour, depth, x, y)).unwrap();
                        }
                    }
                });
            }
            s.spawn(move || {
                while let Ok((colour, depth, x, y)) = rx.recv() {
                    fb.plot_pixel(x, fb.height() - y - 1, colour.r, colour.g, colour.b);
                    fb.plot_depth(x, fb.height() - y - 1, depth);
                }
            });
        });
    }
    pub fn visualise_photons(&self, map: &PhotonMap, env: &Scene, fb: &mut FrameBuffer) {
        thread::scope(|s| {
            let (tx, rx) = channel();
            let thread_num = 8;
            for i in 0..thread_num {
                let tx = tx.clone();
                let starty = i * fb.height() / thread_num;
                let endy = starty + fb.height() / thread_num;

                let width = fb.width();

                s.spawn(move || {
                    for y in starty..endy {
                        for x in 0..width {
                            let mut colour = Colour::default();
                            let mut max_n = 0;
                            for _ in 0..self.samples {
                                let ray = self.get_ray_pixel(x, y);
                                if let Some(best_hit) = env.trace(&ray) {
                                    let (colourtmp, n) = map.visualise(best_hit.position);
                                    colour += colourtmp / self.samples as f32 * n as f32;
                                    max_n = max_n.max(n);
                                }
                            }
                            tx.send((colour, max_n, x, y)).unwrap();
                        }
                    }
                });
            }
            s.spawn(move || {
                let mut max_n = 0;
                while let Ok((colour, n, x, y)) = rx.recv() {
                    max_n = max_n.max(n);
                    fb.plot_pixel(x, fb.height() - y - 1, colour.r, colour.g, colour.b);
                }
                for y in 0..fb.height() {
                    for x in 0..fb.width() {
                        // replot pixels by adjusting for photon neighbour number
                        let c = fb.get_pixel(x, y) / max_n as f32;
                        fb.plot_pixel(x, y, c.r, c.g, c.b);
                    }
                }
            });
        });
    }
}

pub fn random_in_unit_disc() -> Vec3A {
    let mut rng = rand::thread_rng();
    loop {
        let p = Vec3A::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.);
        if p.length_squared() < 1. {
            return p;
        }
    }
}

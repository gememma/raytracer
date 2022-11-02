use std::{sync::mpsc::channel, thread};

use glam::Vec3A;

use crate::{framebuffer::FrameBuffer, ray::Ray, scene::Scene, Vertex};

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
}

impl FullCamera {
    pub fn new(
        fov: f32,
        position: Vertex,
        look: Vertex,
        up: Vec3A,
        width: usize,
        height: usize,
    ) -> Self {
        let w = (look - position).normalize();
        let u = (up.cross(w)).normalize();
        let v = w.cross(u);

        let horizontal = 0.5 * u;
        let vertical = 0.5 * v;
        let bottom_left_pixel = Vec3A::from(position) - horizontal - vertical + fov * w;

        Self {
            width,
            height,
            fov,
            position,
            w,
            v,
            u,
            bottom_left_pixel,
        }
    }

    pub fn get_ray_pixel(&self, x: usize, y: usize) -> Ray {
        Ray::new(
            self.position,
            (self.bottom_left_pixel
                + x as f32 / self.width as f32 * self.u
                + y as f32 / self.height as f32 * self.v
                - self.position)
                .normalize(),
        )
    }
    pub fn render(&self, env: Scene, fb: &mut FrameBuffer) {
        // This method spawns threads that raytrace in parallel for speed
        thread::scope(|s| {
            let (tx, rx) = channel();
            let thread_num = 8;
            for i in 0..thread_num {
                let tx = tx.clone();
                let starty = i * fb.height() / thread_num;
                let endy = starty + fb.height() / thread_num;

                let width = fb.width();
                let env = &env;

                s.spawn(move || {
                    for y in starty..endy {
                        for x in 0..width {
                            let ray = self.get_ray_pixel(x, y);
                            let (colour, depth) = env.raytrace(ray, 5, self.position);

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
}

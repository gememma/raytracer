use crate::scene::Scene;
use crate::{framebuffer::FrameBuffer, ray::Ray, Vertex};
use glam::Vec3A;

use super::Camera;

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleCamera {
    pub width: f32,
    pub height: f32,
    pub fov: f32,
}

impl Default for SimpleCamera {
    fn default() -> Self {
        Self {
            width: 512.,
            height: 512.,
            fov: 0.5,
        }
    }
}

impl SimpleCamera {
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

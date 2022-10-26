use crate::scene::Scene;
use crate::{framebuffer::FrameBuffer, ray::Ray, Vertex};
use glam::Vec3A;

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
        for y in 0..fb.height() {
            for x in 0..fb.width() {
                let ray = self.get_ray_pixel(x, y);

                let (colour, depth) = env.raytrace(ray, 5, self.position);

                fb.plot_pixel(x, fb.height() - y - 1, colour.r, colour.g, colour.b);
                fb.plot_depth(x, fb.height() - y - 1, depth);
            }
        }
    }
}

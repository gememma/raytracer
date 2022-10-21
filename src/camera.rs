use crate::framebuffer::FrameBuffer;
use crate::scene::Scene;

pub mod full;
pub mod simple;

pub trait Camera {
    fn render(&self, env: Scene, fb: &mut FrameBuffer);
}

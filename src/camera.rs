use crate::framebuffer::FrameBuffer;
use crate::scene::Scene;

pub mod full;
pub mod simple;

pub trait Camera: Default {
    fn render(&self, env: Scene, fb: &mut FrameBuffer);
}

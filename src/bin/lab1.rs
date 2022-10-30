use raytracer::framebuffer::FrameBuffer;
use raytracer::linedrawer::draw_line;
use std::f32::consts::PI;

fn main() {
    // create framebuffer
    let mut f = FrameBuffer::new(512, 512);

    // generate radial lines
    for line in 0..64 {
        let i = PI / 32. * line as f32;
        let x = i.cos();
        let y = i.sin();

        draw_line(
            &mut f,
            (256. + (48. * x)) as i32,
            (256. + (48. * y)) as i32,
            (256. + (240. * x)) as i32,
            (256. + (240. * y)) as i32,
        );
    }

    // output framebuffer
    fb.write_rgb_png("test.png")
        .expect("failed to write RGB output to PNG file");
}

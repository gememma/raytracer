use glam::{Affine3A, Vec3A};
use raytracer::framebuffer::FrameBuffer;
use raytracer::linedrawer::draw_line;
use raytracer::object::polymesh::PolyMesh;
use raytracer::object::Object;
use std::process::exit;

fn main() {
    // create framebuffer
    let mut f = FrameBuffer::new(1024, 1024);

    // read in model
    let mut mesh = PolyMesh::new("teapot_smaller.ply", false, false);
    let t = Affine3A::from_cols(
        Vec3A::new(1., 0., 0.),
        Vec3A::new(0., 0., 1.),
        Vec3A::new(0., 1., 0.),
        Vec3A::new(0., -2.7, 5.),
    );
    mesh.apply_transform(t);

    // draw each triangle in the model
    for i in 0..mesh.triangle_count {
        // project the points in 3D space to 2D canvas
        let x0 = (mesh.vertices[mesh.triangle_indices[i][0]].x
            / mesh.vertices[mesh.triangle_indices[i][0]].z)
            * 700.
            + 512.;
        let y0 = (mesh.vertices[mesh.triangle_indices[i][0]].y
            / mesh.vertices[mesh.triangle_indices[i][0]].z)
            * -700.
            + 256.;
        let x1 = (mesh.vertices[mesh.triangle_indices[i][1]].x
            / mesh.vertices[mesh.triangle_indices[i][1]].z)
            * 700.
            + 512.;
        let y1 = (mesh.vertices[mesh.triangle_indices[i][1]].y
            / mesh.vertices[mesh.triangle_indices[i][1]].z)
            * -700.
            + 256.;
        let x2 = (mesh.vertices[mesh.triangle_indices[i][2]].x
            / mesh.vertices[mesh.triangle_indices[i][2]].z)
            * 700.
            + 512.;
        let y2 = (mesh.vertices[mesh.triangle_indices[i][2]].y
            / mesh.vertices[mesh.triangle_indices[i][2]].z)
            * -700.
            + 256.;

        draw_line(&mut f, x0 as i32, y0 as i32, x1 as i32, y1 as i32);
        draw_line(&mut f, x1 as i32, y1 as i32, x2 as i32, y2 as i32);
        draw_line(&mut f, x2 as i32, y2 as i32, x0 as i32, y0 as i32);
    }

    // output framebuffer
    f.write_rgb_file("test.ppm")
        .expect("Writing RGB file failed");
}

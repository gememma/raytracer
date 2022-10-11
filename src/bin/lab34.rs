//! This is the top level for the program you need to create for lab three and four.
//!
//! You can run this file using `cargo`:
//!
//! ```
//! cargo run --bin lab34
//! ```
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

use raytracer::camera::full::FullCamera;
use raytracer::{
    camera::{simple::SimpleCamera, Camera},
    colour::Colour,
    framebuffer::FrameBuffer,
    light::directional::DirectionalLight,
    material::phong::Phong,
    object::{polymesh::PolyMesh, sphere::Sphere, Object},
    scene::Scene,
    transform::Transform,
    vector::Vector,
    vertex::Vertex,
};

/// You will find it useful during development/debugging to create multiple functions that fill out
/// the scene.
fn build_scene(scene: &mut Scene) {
    // The following transform allows 4D homogeneous coordinates to be transformed. It moves the
    // supplied teapot model to somewhere visible.
    let transform = Transform::new(
        [1., 0., 0., 0.],
        [0., 0., 1., -2.7],
        [0., 1., 0., 5.],
        [0., 0., 0., 1.],
    );

    // Read in the teapot model.
    let mut pm = PolyMesh::new("teapot_smaller.ply", false);
    // let mut pm = PolyMesh::new("teapot.ply", false);
    pm.apply_transform(&transform);

    let mut sphere = Sphere::new(Vertex::from_xyz(0., 1.5, 3.), 0.4);

    // let dl = DirectionalLight::new(
    //     Vector::new(1.01, -1., 1.),
    //     Colour::from_rgba(1., 1., 1., 0.),
    // );
    //
    // scene.add_light(dl);

    // let bp1 = Phong::new(
    //     Colour::from_rgb(0.2, 0., 0.),
    //     Colour::from_rgb(0.4, 0., 0.),
    //     Colour::from_rgb(0.4, 0.4, 0.4),
    //     40.,
    // );
    // let bp2 = Phong::new(
    //     Colour::from_rgb(0.01, 0.01, 0.01),
    //     Colour::from_rgb(0., 0., 0.),
    //     Colour::from_rgb(0.5, 0.5, 0.5),
    //     40.,
    // );

    // pm.set_material(Box::new(bp1));

    scene.add_object(pm);

    // sphere.set_material(Box::new(bp2));

    scene.add_object(sphere);
}

// This is the entry point function to the program.
fn main() {
    let width = 512;
    let height = 512;
    // Create a framebuffer
    let mut fb = FrameBuffer::new(width, height);

    // Create a scene
    let mut scene = Scene::default();

    // Setup the scene
    build_scene(&mut scene);

    // Declare a camera
    // let camera = SimpleCamera::with_fov(0.5);
    let camera = FullCamera::new(
        0.5,
        Vertex::from_xyz(0., 0., 0.),
        Vertex::from_xyz(0., 0., 1.),
        Vector::new(0., 1., 0.),
        width,
        height,
    );

    // Camera generates rays for each pixel in the framebuffer and records colour + depth.
    camera.render(&scene, &mut fb);

    // Output the framebuffer colour and depth as two images
    fb.write_rgb_file("test.ppm")
        .expect("failed to write RGB output to PPM file");
    fb.write_depth_file("depth.ppm")
        .expect("failed to write depth output to PPM file");

    // eprintln!("\nDone.");
}

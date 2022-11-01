use glam::{Affine3A, Vec3A};
use rand::Rng;
use raytracer::fullcamera::FullCamera;
use raytracer::{
    colour::Colour,
    framebuffer::FrameBuffer,
    light::directional::Directional,
    material::phong::Phong,
    object::{polymesh::PolyMesh, sphere::Sphere, Object},
    scene::Scene,
    Vertex,
};
use std::f32::consts::PI;

fn build_scene(scene: &mut Scene) {
    // Create objects
    let transform = Affine3A::from_cols(
        Vec3A::new(1., 0., 0.),
        Vec3A::new(0., 0., 1.),
        Vec3A::new(0., 1., 0.),
        Vec3A::new(0., -2.7, 4.),
    );
    let t2 = Affine3A::from_rotation_x(PI / 2.);

    // Read in the bigger teapot model
    // let mut pm = PolyMesh::new("teapot.ply", true, true);
    let mut pm = PolyMesh::new("teapot_smaller.ply", true, false);
    // pm.apply_transform(t2);
    pm.apply_transform(transform);

    let mut ground = Sphere::new(Vertex::new(0., -103.5, -1.), 100.);

    // Create lighting
    let dl = Directional::new(Vec3A::new(0.5, -1., 0.5), Colour::from_rgba(1., 1., 1., 0.));
    scene.add_light(dl);

    // Create materials manually
    let bp1 = Phong::new(
        Colour::from_rgb(0.2, 0., 0.),
        Colour::from_rgb(0.4, 0., 0.),
        Colour::from_rgb(0.5, 0.5, 0.5),
        40.,
    );
    pm.set_material(Box::new(bp1));
    scene.add_object(pm);

    let bp2 = Phong::new(
        Colour::from_rgb(0., 0.3, 0.1),
        Colour::from_rgb(0., 0.6, 0.2),
        Colour::from_rgb(0.4, 0.4, 0.4),
        40.,
    );
    ground.set_material(Box::new(bp2));
    scene.add_object(ground);

    // Create 9 random colour/size/position spheres
    for _ in 1..15 {
        let mut sphere = spawn_sphere(Vec3A::new(-1., 0., 5.), Vec3A::new(1., 2., 7.), 0.6);
        let c = Colour::random(0.1, 0.7);
        sphere.set_material(Box::new(Phong::new(
            c * 0.6,
            c,
            Colour::from_rgb(0.5, 0.5, 0.5),
            40.,
        )));
        scene.add_object(sphere);
    }
}

fn main() {
    // Create a framebuffer
    // let mut fb = FrameBuffer::new(2048, 2048);
    let mut fb = FrameBuffer::default();

    // Create a scene
    let mut scene = Scene::default();

    // Setup the scene
    build_scene(&mut scene);

    // Declare a camera
    let camera = FullCamera::new(
        1.,
        Vertex::new(0., 1., -4.),
        Vertex::new(0., 0.5, 1.),
        Vec3A::new(0., 1., 0.),
        fb.width(),
        fb.height(),
    );

    // Camera generates rays for each pixel in the framebuffer and records colour + depth.
    camera.render(scene, &mut fb);

    // Output the framebuffer colour and depth as two images
    fb.write_rgb_png("test.png")
        .expect("failed to write RGB output to PNG file");
    fb.write_depth_png("depth.png")
        .expect("failed to write depth output to PNG file");
}

fn spawn_sphere(min_pos: Vec3A, max_pos: Vec3A, max_rad: f32) -> Sphere {
    // generate a randomly sized and positioned sphere
    let x = rand::thread_rng().gen_range(min_pos.x..max_pos.x);
    let y = rand::thread_rng().gen_range(min_pos.y..max_pos.y);
    let z = rand::thread_rng().gen_range(min_pos.z..max_pos.z);
    let center = Vertex::new(x, y, z);
    Sphere::new(center, rand::thread_rng().gen_range(0.2..max_rad))
}

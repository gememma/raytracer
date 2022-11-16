use glam::{Affine3A, Vec3A};
use rand::Rng;
use raytracer::{
    colour::Colour,
    framebuffer::FrameBuffer,
    fullcamera::FullCamera,
    light::{directional::Directional, point::Point},
    material::{dielectric::Dielectric, metallic::Metallic, phong::Phong},
    object::{plane::Plane, polymesh::PolyMesh, sphere::Sphere, triangle::Triangle, Object},
    scene::Scene,
    Vertex,
};

fn main() {
    // Create a framebuffer
    // let mut fb = FrameBuffer::new(2048, 2048);
    let mut fb = FrameBuffer::default();

    // Create a scene
    let mut scene = Scene::default();

    // Setup the scene
    // build_scene(&mut scene);
    build_c_box(&mut scene);

    // Declare a camera
    let camera = FullCamera::new(
        1.,
        Vertex::new(0., 0., 0.),
        Vertex::new(0., 0., 8.),
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

fn build_c_box(scene: &mut Scene) {
    // materials
    let mat_white = Phong::new(
        Colour::from_rgb(0.1, 0.1, 0.1),
        Colour::from_rgb(0.6, 0.6, 0.6),
        Colour::from_rgb(0.4, 0.4, 0.4),
        40.,
    );
    let mat_red = Phong::new(
        Colour::from_rgb(0.2, 0., 0.),
        Colour::from_rgb(0.4, 0., 0.),
        Colour::from_rgb(0.5, 0.5, 0.5),
        40.,
    );
    let mat_green = Phong::new(
        Colour::from_rgb(0., 0.2, 0.),
        Colour::from_rgb(0., 0.4, 0.),
        Colour::from_rgb(0.5, 0.5, 0.5),
        40.,
    );
    let mat_glass = Dielectric::new(1.52);
    // floor
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., -3., 4.),
            Vec3A::new(3., -3., 4.),
        ],
        Box::new(mat_white.clone()),
    ));
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(3., -3., 4.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        Box::new(mat_white.clone()),
    ));

    // ceiling
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(-3., 3., 4.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        Box::new(mat_white.clone()),
    ));
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., 3., 4.),
            Vec3A::new(-3., 3., 4.),
        ],
        Box::new(mat_white.clone()),
    ));

    // left wall
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(-3., 3., 4.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        Box::new(mat_red.clone()),
    ));
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., -3., 4.),
            Vec3A::new(-3., 3., 4.),
        ],
        Box::new(mat_red.clone()),
    ));

    // right wall
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., 3., 4.),
            Vec3A::new(3., -3., 4.),
        ],
        Box::new(mat_green.clone()),
    ));
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(3., -3., 4.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        Box::new(mat_green.clone()),
    ));

    // back wall
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        Box::new(mat_white.clone()),
    ));
    scene.add_object(Triangle::new_with_material(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        Box::new(mat_white.clone()),
    ));

    let mut spherel = Sphere::new(Vec3A::new(-0.8, -0.5, 5.), 0.4);
    let mut spherem = Sphere::new(Vec3A::new(0., -0.5, 5.), 0.4);
    let mut spherer = Sphere::new(Vec3A::new(0.8, -0.5, 5.), 0.4);
    spherel.set_material(Box::new(mat_glass.clone()));
    spherem.set_material(Box::new(Metallic::new(
        Colour::from_rgb(0.6, 0.6, 0.6),
        40.,
    )));
    spherer.set_material(Box::new(mat_white.clone()));
    scene.add_object(spherel);
    scene.add_object(spherem);
    scene.add_object(spherer);

    // lights
    scene.add_light(Point::new(
        Vec3A::new(0., 2., 4.),
        Colour::from_rgba(1., 1., 1., 0.),
    ));
}

#[allow(dead_code)]
fn spawn_sphere(min_pos: Vec3A, max_pos: Vec3A, max_rad: f32) -> Sphere {
    // generate a randomly sized and positioned sphere
    let x = rand::thread_rng().gen_range(min_pos.x..max_pos.x);
    let y = rand::thread_rng().gen_range(min_pos.y..max_pos.y);
    let z = rand::thread_rng().gen_range(min_pos.z..max_pos.z);
    let center = Vertex::new(x, y, z);
    Sphere::new(center, rand::thread_rng().gen_range(0.2..max_rad))
}

#[allow(dead_code)]
fn build_scene(scene: &mut Scene) {
    // Create objects
    let transform = Affine3A::from_cols(
        Vec3A::new(1., 0., 0.),
        Vec3A::new(0., 0., 1.),
        Vec3A::new(0., 1., 0.),
        Vec3A::new(0., -2.7, 4.),
    );

    let mut pm = PolyMesh::new("teapot_smaller.ply", true, false);
    pm.apply_transform(transform);

    let mut ground = Plane::new(Vec3A::new(0., 1., 0.), Vertex::new(0., -3.5, 0.));
    let mut sphere = Sphere::new(Vertex::new(2.1, 1.2, 4.), 0.8);

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
        Colour::from_rgb(0., 0.15, 0.1),
        Colour::from_rgb(0., 0.3, 0.15),
        Colour::from_rgb(0.4, 0.4, 0.4),
        40.,
    );
    ground.set_material(Box::new(bp2));
    scene.add_object(ground);

    let met1 = Metallic::new(Colour::from_rgb(0.6, 0.8, 0.8), 40.);
    sphere.set_material(Box::new(met1));
    scene.add_object(sphere);

    // Create 9 random colour/size/position spheres
    for _ in 1..15 {
        let mut sphere = spawn_sphere(Vec3A::new(-1., 1., 3.), Vec3A::new(1., 3., 7.), 0.6);
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

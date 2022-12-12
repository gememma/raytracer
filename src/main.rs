use glam::{Affine3A, Vec3, Vec3A};
use rand::Rng;
use raytracer::{
    colour::Colour,
    framebuffer::FrameBuffer,
    fullcamera::FullCamera,
    light::{directional::Directional, point::Point},
    material::{
        dielectric::Dielectric, diffuse::Diffuse, metallic::Metallic, normalshading::NormalShading,
        phong::Phong,
    },
    object::{
        csg::{Csg, Op},
        plane::Plane,
        polymesh::PolyMesh,
        quadratic::Quadratic,
        sphere::Sphere,
        triangle::Triangle,
        Object,
    },
    photonmap::PhotonMap,
    scene::Scene,
    Vertex,
};

fn main() {
    // create framebuffers
    let mut fb = FrameBuffer::default();
    let mut photons_fb = FrameBuffer::default();

    // create a scene
    let mut scene = Scene::default();

    // setup the scene
    build_scene(&mut scene);

    // setup a Cornell box for debugging
    // build_c_box(&mut scene);

    // define a camera
    let camera = FullCamera::new(
        1.,
        Vertex::new(0., 0., 0.),
        Vertex::new(0., 0., 6.),
        Vec3A::new(0., 1., 0.),
        fb.width(),
        fb.height(),
        100,
        0.,
    );

    // build caustics map
    // let caustic_pmap = PhotonMap::build_caustics(&scene);

    // camera generates rays for each pixel in the framebuffer and records colour + depth.
    let pmap = PhotonMap::build(&scene);
    camera.render(&scene, &mut fb, &pmap);

    // build the visualisation for the photon map for debugging
    // camera.visualise_photons(&pmap, &scene, &mut photons_fb);

    // add the caustics map to the frame buffer
    // fb = fb.add_caustics(&photons_fb);

    // output the framebuffer colour and depth, and the photon map visualisation
    fb.write_rgb_png("test.png")
        .expect("failed to write RGB output to PNG file");
    fb.write_depth_png("depth.png")
        .expect("failed to write depth output to PNG file");

    // write photon map visualisation for debugging
    // photons_fb.write_rgb_png("photons.png").expect("f");
}

#[allow(dead_code)]
fn build_c_box(scene: &mut Scene) {
    // builds a more traditional Cornell box scene for debugging
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

    let mut spherel = Sphere::new(Vec3A::new(0., -0.4, 3.5), 0.4);
    let mut spherem = Sphere::new(Vec3A::new(0., -0.5, 5.), 0.4);
    let mut spherer = Sphere::new(Vec3A::new(0.8, -0.5, 5.), 0.4);
    spherel.set_material(Box::new(mat_glass.clone()));

    spherem.set_material(Box::new(mat_white.clone()));
    spherer.set_material(Box::new(Metallic::new(Colour::from_rgb(0.7, 0.7, 0.7), 0.)));
    scene.add_object(spherel);
    scene.add_object(spherem);
    scene.add_object(spherer);

    // lights
    scene.add_light(Point::new(
        Vec3A::new(0., 2., 3.),
        Colour::from_rgba(1., 1., 1., 0.),
    ));
}

fn spawn_sphere(min_pos: Vec3A, max_pos: Vec3A, max_rad: f32) -> Sphere {
    // generate a randomly sized and positioned sphere
    let x = rand::thread_rng().gen_range(min_pos.x..max_pos.x);
    let y = rand::thread_rng().gen_range(min_pos.y..max_pos.y);
    let z = rand::thread_rng().gen_range(min_pos.z..max_pos.z);
    let center = Vertex::new(x, y, z);
    Sphere::new(center, rand::thread_rng().gen_range(0.2..max_rad))
}

fn build_scene(scene: &mut Scene) {
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

    // create teapot
    let mut pm = PolyMesh::new("teapot_smaller.ply", true, false);
    pm.apply_transform(Affine3A::from_scale(Vec3::new(0.6, 0.6, 0.6)));
    pm.apply_transform(Affine3A::from_cols(
        Vec3A::new(1., 0., 0.),
        Vec3A::new(0., 0., 1.),
        Vec3A::new(0., 1., 0.),
        Vec3A::new(0., -2.7, 7.),
    ));
    pm.set_material(Box::new(Metallic::new(
        Colour::from_rgb(0.7, 0.6, 0.85),
        0.,
    )));
    scene.add_object(pm);

    // create random colour/size/position spheres
    for _ in 1..10 {
        let mut sphere = spawn_sphere(Vec3A::new(-2., -1., 5.), Vec3A::new(2., 2., 9.), 0.6);
        let c = Colour::random(0.1, 0.7);
        sphere.set_material(Box::new(Phong::new(
            c * 0.6,
            c,
            Colour::from_rgb(0.5, 0.5, 0.5),
            40.,
        )));
        scene.add_object(sphere);
    }

    // create glass spheres
    let mut gl_sphere1 = Sphere::new(Vec3A::new(-1.8, -2.5, 6.5), 0.6);
    gl_sphere1.set_material(Box::new(mat_glass.clone()));
    scene.add_object(gl_sphere1);
    let mut gl_sphere2 = Sphere::new(Vec3A::new(1.4, -2.7, 7.), 0.3);
    gl_sphere2.set_material(Box::new(mat_glass.clone()));
    scene.add_object(gl_sphere2);

    // create Cornell box walls
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

    // create point light
    scene.add_light(Point::new(
        Vec3A::new(0., 2., 3.),
        Colour::from_rgba(1., 1., 1., 0.),
    ));
}

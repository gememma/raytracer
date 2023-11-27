use glam::{Affine3A, Vec3, Vec3A};
use raytracer::{
    colour::Colour,
    framebuffer::FrameBuffer,
    fullcamera::FullCamera,
    light::point::Point,
    material::{dielectric::Dielectric, diffuse::Diffuse, metallic::Metallic, phong::Phong},
    object::{
        csg::{Csg, Op},
        polymesh::PolyMesh,
        sphere::Sphere,
        triangle::Triangle,
        Object,
    },
    photonmap::PhotonMap,
    scene::Scene,
    Vertex,
};

fn main() {
    // create framebuffer for render output
    let mut fb = FrameBuffer::default();

    // create a scene
    let mut scene = Scene::default();

    // setup the scene
    // build_scene(&mut scene);
    build_final_scene(&mut scene);

    // setup a Cornell box for debugging
    // build_c_box(&mut scene);

    // define a camera
    let camera = FullCamera::new(
        1.,
        Vertex::new(0., 0., 0.),
        Vertex::new(0., 0., 8.),
        Vec3A::new(0., 1., 0.),
        fb.width(),
        fb.height(),
        1000,
        0.,
    );

    // build caustics map WARNING: SLOW
    // let mut photons_fb = FrameBuffer::default();
    // let caustic_pmap = PhotonMap::build(&scene);

    // camera generates rays for each pixel in the framebuffer and records colour + depth.
    let pmap = PhotonMap::build(&scene);
    camera.render(&scene, &mut fb, &pmap);

    // build the visualisation for the photon map for debugging WARNING: SLOW
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

fn build_final_scene(scene: &mut Scene) {
    // create materials
    let mat_white = Diffuse::new(Colour::from_rgb(0.6, 0.6, 0.6));
    let mat_red = Diffuse::new(Colour::from_rgb(0.6, 0., 0.));
    let mat_green = Diffuse::new(Colour::from_rgb(0., 0.6, 0.));
    let mat_glass = Dielectric::new(1.52, Colour::from_rgb(0.95, 0.95, 0.95));
    let mat_metal = Metallic::new(Colour::from_rgb(0.8, 0.8, 1.), 0.);
    let mat_metal_r = Metallic::new(Colour::from_rgb(0.8, 0.8, 1.), 0.05);

    // floor
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., -3., 4.),
            Vec3A::new(3., -3., 4.),
        ],
        mat_metal_r,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., -3., 4.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        mat_metal_r,
    ));

    // ceiling
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., 3., 4.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., 3., 4.),
            Vec3A::new(-3., 3., 4.),
        ],
        mat_white,
    ));

    // left wall
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., 3., 4.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        mat_red,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., -3., 4.),
            Vec3A::new(-3., 3., 4.),
        ],
        mat_red,
    ));

    // right wall
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., 3., 4.),
            Vec3A::new(3., -3., 4.),
        ],
        mat_green,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., -3., 4.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        mat_green,
    ));

    // back wall
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        mat_white,
    ));

    // pedestal box
    // front face
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-1.5, -0.5, 7.25),
            Vec3A::new(0., -0.5, 8.),
            Vec3A::new(0., -3., 8.),
        ],
        mat_metal,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(0., -3., 8.),
            Vec3A::new(-1.5, -3., 7.25),
            Vec3A::new(-1.5, -0.5, 7.25),
        ],
        mat_metal,
    ));
    // left face
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-2.25, -0.5, 8.75),
            Vec3A::new(-1.5, -0.5, 7.25),
            Vec3A::new(-1.5, -3., 7.25),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-1.5, -3., 7.25),
            Vec3A::new(-2.25, -3., 8.75),
            Vec3A::new(-2.25, -0.5, 8.75),
        ],
        mat_white,
    ));
    // top face
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-0.75, -0.5, 9.5),
            Vec3A::new(0., -0.5, 8.),
            Vec3A::new(-1.5, -0.5, 7.25),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-1.5, -0.5, 7.25),
            Vec3A::new(-2.25, -0.5, 8.75),
            Vec3A::new(-0.75, -0.5, 9.5),
        ],
        mat_white,
    ));

    // teapot polymesh
    let mut pm = PolyMesh::new("teapot_smaller.ply", true, false, mat_glass);
    pm.apply_transform(Affine3A::from_scale(Vec3::new(0.6, 0.6, 0.6)));
    pm.apply_transform(Affine3A::from_rotation_z(0.46));
    pm.apply_transform(Affine3A::from_cols(
        Vec3A::new(1., 0., 0.),
        Vec3A::new(0., 0., 1.),
        Vec3A::new(0., 1., 0.),
        Vec3A::new(-1., -0.525, 8.475),
    ));
    scene.add_object(pm);

    // large sphere
    scene.add_object(Sphere::new(Vec3A::new(1.65, -2.05, 7.6), 0.95, mat_metal));

    // other spheres
    scene.add_object(Sphere::new(Vec3A::new(-2.3, -2.5, 7.5), 0.5, mat_white));

    let sphere_sm_1 = Sphere::new(Vec3A::new(0., -2.75, 6.75), 0.25, mat_glass);
    let sphere_sm_2 = Sphere::new(Vec3A::new(0.25, -2.75, 6.75), 0.25, mat_glass);
    let sphere_sm_3 = Sphere::new(Vec3A::new(0.5, -2.75, 6.75), 0.25, mat_glass);
    let sphere_sm_4 = Sphere::new(Vec3A::new(0.75, -2.75, 6.75), 0.25, mat_glass);

    let glass_obj_p1 = Csg::new_branch(sphere_sm_1, sphere_sm_2, Op::Union);
    let glass_obj_p2 = Csg::new_branch(sphere_sm_3, sphere_sm_4, Op::Union);
    let mut glass_obj = Csg::new_branch(glass_obj_p1, glass_obj_p2, Op::Union);

    let t = Affine3A::from_translation(Vec3::new(-0.5, 0., 0.3)) * Affine3A::from_rotation_y(0.05);
    glass_obj.apply_transform(t);
    scene.add_object(glass_obj);

    // pyramid
    let t = Affine3A::from_translation(Vec3::new(-1.5, 0., 6.8)) * Affine3A::from_rotation_y(0.1);
    scene.add_object(spawn_pyramid(
        Vec3A::new(0.5, -3., 0.5),
        Vec3A::new(0.5, -3., -0.5),
        Vec3A::new(-0.5, -3., -0.5),
        Vec3A::new(-0.5, -3., 0.5),
        Vec3A::new(0., -2., 0.),
        t,
    ));

    scene.add_light(Point::new(
        Vec3A::new(0.5, 2.8, 6.),
        Colour::from_rgb(0.8, 0.8, 0.8),
    ));
    scene.add_light(Point::new(
        Vec3A::new(-0.5, 2.8, 6.),
        Colour::from_rgb(0.8, 0.8, 0.8),
    ));
}

fn spawn_pyramid(a: Vec3A, b: Vec3A, c: Vec3A, d: Vec3A, e: Vec3A, t: Affine3A) -> Csg {
    let mat_blue = Diffuse::new(Colour::from_rgb(0.25, 0.25, 0.85));
    let part1 = Csg::new_branch(
        Triangle::new([a, b, e], mat_blue),
        Triangle::new([b, c, e], mat_blue),
        Op::Union,
    );
    let part2 = Csg::new_branch(
        Triangle::new([c, d, e], mat_blue),
        Triangle::new([d, a, e], mat_blue),
        Op::Union,
    );
    let mut pyramid = Csg::new_branch(part1, part2, Op::Union);
    pyramid.apply_transform(t);
    pyramid
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
    let mat_glass = Dielectric::new(1.52, Colour::from_rgb(1., 1., 1.));

    // floor
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., -3., 4.),
            Vec3A::new(3., -3., 4.),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., -3., 4.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        mat_white,
    ));

    // ceiling
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., 3., 4.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., 3., 4.),
            Vec3A::new(-3., 3., 4.),
        ],
        mat_white,
    ));

    // left wall
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., 3., 4.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        mat_red,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., -3., 4.),
            Vec3A::new(-3., 3., 4.),
        ],
        mat_red,
    ));

    // right wall
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., 3., 4.),
            Vec3A::new(3., -3., 4.),
        ],
        mat_green,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., -3., 4.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        mat_green,
    ));

    // back wall
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        mat_white,
    ));

    let spherel = Sphere::new(Vec3A::new(0., -0.4, 3.5), 0.4, mat_glass);
    let spherem = Sphere::new(Vec3A::new(0., -0.5, 5.), 0.4, mat_white);
    let spherer = Sphere::new(
        Vec3A::new(0.8, -0.5, 5.),
        0.4,
        Metallic::new(Colour::from_rgb(0.7, 0.7, 0.7), 0.),
    );

    scene.add_object(spherel);
    scene.add_object(spherem);
    scene.add_object(spherer);

    // lights
    scene.add_light(Point::new(
        Vec3A::new(0., 2., 3.),
        Colour::from_rgba(1., 1., 1., 0.),
    ));
}

#[allow(dead_code)]
fn build_scene(scene: &mut Scene) {
    // create materials
    let mat_white = Diffuse::new(Colour::from_rgb(0.6, 0.6, 0.6));
    let mat_red = Diffuse::new(Colour::from_rgb(0.6, 0., 0.));
    let mat_green = Diffuse::new(Colour::from_rgb(0., 0.6, 0.));
    let mat_glass = Dielectric::new(1.52, Colour::from_rgb(1., 0.9, 0.8));
    let mat_metal = Metallic::new(Colour::from_rgb(0.8, 0.8, 1.), 0.);

    // create teapot
    let mut pm = PolyMesh::new("teapot_smaller.ply", true, false, mat_glass);
    pm.apply_transform(Affine3A::from_scale(Vec3::new(0.6, 0.6, 0.6)));
    pm.apply_transform(Affine3A::from_rotation_z(0.5));
    pm.apply_transform(Affine3A::from_cols(
        Vec3A::new(1., 0., 0.),
        Vec3A::new(0., 0., 1.),
        Vec3A::new(0., 1., 0.),
        Vec3A::new(0., -2.7, 7.),
    ));
    scene.add_object(pm);

    // create glass and metal spheres
    scene.add_object(Sphere::new(Vec3A::new(-1.8, -2.1, 9.), 0.9, mat_metal));
    scene.add_object(Sphere::new(Vec3A::new(1.4, -2.7, 7.), 0.3, mat_glass));

    // create Cornell box walls
    // floor
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., -3., 4.),
            Vec3A::new(3., -3., 4.),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., -3., 4.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        mat_white,
    ));

    // ceiling
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., 3., 4.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., 3., 4.),
            Vec3A::new(-3., 3., 4.),
        ],
        mat_white,
    ));

    // left wall
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., 3., 4.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        mat_red,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., -3., 4.),
            Vec3A::new(-3., 3., 4.),
        ],
        mat_red,
    ));

    // right wall
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., 3., 4.),
            Vec3A::new(3., -3., 4.),
        ],
        mat_green,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., -3., 4.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        mat_green,
    ));

    // back wall
    scene.add_object(Triangle::new(
        [
            Vec3A::new(-3., -3., 10.),
            Vec3A::new(-3., 3., 10.),
            Vec3A::new(3., 3., 10.),
        ],
        mat_white,
    ));
    scene.add_object(Triangle::new(
        [
            Vec3A::new(3., 3., 10.),
            Vec3A::new(3., -3., 10.),
            Vec3A::new(-3., -3., 10.),
        ],
        mat_white,
    ));

    // create point light
    scene.add_light(Point::new(
        Vec3A::new(0., 2., 3.),
        Colour::from_rgba(1., 1., 1., 0.),
    ));
}

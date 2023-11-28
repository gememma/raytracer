# raytracer

<p align="center">
    <img src="report/images/csgunion.png" width="250" alt=""/>
    <img src="report/images/smoothedpolym.png" width="250" alt=""/>
    <img src="report/images/depthoffield.png" width="250" alt=""/>
<p>

## Project

This application is a ray tracer that renders a 3D scene containing different objects in various materials. The [report](report/cm30075report.pdf) contains technical details of the implementation. This software was created with the eventual goal of producing raytraced images using a photon map and including caustic effects through dielectric material. The project is not fully complete, and the photon maps are not currently used in the production of the image.

### Features

- [Dielectric](src/material/dielectric.rs), [diffuse](src/material/diffuse.rs), [metallic](src/material/metallic.rs) and [Phong shaded](src/material/phong.rs) materials
- [Direction](src/light/directional.rs) and [point](src/light/point.rs) lights
- [CSG](src/object/csg.rs), [polymesh](src/object/polymesh.rs), [quadratic](src/object/quadratic.rs), [sphere](src/object/sphere.rs) and [triangle](src/object/triangle.rs) objects
- Polymesh object creation from a file, smoothed or unsmoothed
- Parallelised image rendering
- A fully adjustable [camera](src/fullcamera.rs)

## Usage

Rust is required to run this project. You can install the Rust compiler using [Rustup](https://rustup.rs).

Run `cargo run` to build and run the project. Beware that without changes, this will generate a  very high quality image (1024 x 1024) and take a long time. The output image will appear in the root directory as `test.png` and may look something like [this example](images/exampleoutput.png).
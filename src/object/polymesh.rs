//! [`PolyMesh`], an [`Object`] that reads and intersects with triangle meshes.
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

use crate::{
    hit::Hit,
    material::{falsecolour::FalseColour, Material},
    ray::Ray,
    transform::Transform,
    vertex::Vertex,
};

use super::Object;

type TriangleIndex = [usize; 3];

/// `PolyMesh` reads and intersects with triangle meshes.
#[derive(Debug)]
pub struct PolyMesh {
    pub vertex_count: usize,
    pub triangle_count: usize,
    pub vertex: Vec<Vertex>,
    pub triangle: Vec<TriangleIndex>,
    pub smoothing: bool,
    material: Box<dyn Material>,
}

impl PolyMesh {
    /// This is the equivalent of the two-argument constructor from the C++ version.
    pub fn new(_file: &str, _smooth: bool) -> Self {
        // Remove the #[allow(unreachable_code)] once you have implemented this function.
        #[allow(unreachable_code)]
        PolyMesh {
            vertex_count: todo!("you must implement reading a polymesh from a file"),
            triangle_count: todo!("you must implement reading a polymesh from a file"),
            vertex: todo!("you must implement reading a polymesh from a file"),
            triangle: todo!("you must implement reading a polymesh from a file"),
            smoothing: todo!("you must implement reading a polymesh from a file"),
            material: Box::new(FalseColour::default()),
        }
    }
}

impl Object for PolyMesh {
    fn material(&self) -> &dyn Material {
        &*self.material
    }

    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    fn intersection(&self, _ray: &Ray) -> Vec<Hit> {
        todo!("you must implement polymesh-ray intersection")
    }

    fn apply_transform(&mut self, _transform: &Transform) {
        todo!("you must implement applying a transform to a polymesh")
    }
}

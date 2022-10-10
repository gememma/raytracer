//! [`Plane`], an [`Object`] that is an infinite plane that has volume.
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
    transform::{Apply, Transform},
    vector::Vector,
    vertex::Vertex,
};

use super::Object;

/// `Plane` is an infinite plane that has volume. It returns +/- big number on the inside of the
/// ray.
#[derive(Debug)]
pub struct Plane {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    material: Box<dyn Material>,
}

impl Plane {
    pub fn new(a: f32, b: f32, c: f32, d: f32) -> Self {
        Self {
            a,
            b,
            c,
            d,
            material: Box::new(FalseColour::default()),
        }
    }
}

impl Object for Plane {
    fn material(&self) -> &dyn Material {
        &*self.material
    }

    fn set_material(&mut self, material: Box<dyn Material>) {
        self.material = material;
    }

    fn intersection(&self, ray: &Ray) -> Vec<Hit> {
        let u =
            self.a * ray.position.x + self.b * ray.position.y + self.c * ray.position.z + self.d;
        let v = self.a * ray.direction.x + self.b * ray.direction.y + self.c * ray.direction.z;

        if v == 0. {
            // parallel to plane
            if u < 0. {
                // is inside
                let hit1 = Hit {
                    t: -10000000000., // infinity
                    entering: true,
                    what: self,
                    position: Vertex::default(),
                    normal: Vector::default(),
                };
                let hit2 = Hit {
                    t: 10000000000., // infinity
                    entering: false,
                    what: self,
                    position: Vertex::default(),
                    normal: Vector::default(),
                };
                return vec![hit1, hit2];
            } else {
                // is outside
                return vec![];
            }
        }

        if v > 0. {
            let t = u / -v;
            let hit1 = Hit {
                t: -10000000000., // infinity
                entering: true,
                what: self,
                position: Vertex::default(),
                normal: Vector::default(),
            };
            let mut hit2 = Hit {
                t, // infinity (this is the C++ comment, but I'm pretty sure this is not infinity)
                entering: false,
                what: self,
                position: ray.position + t * ray.direction,
                normal: Vector::new(self.a, self.b, self.c),
            };
            if hit2.normal.dot(ray.direction) > 0. {
                hit2.normal.negate();
            }
            return vec![hit1, hit2];
        } else {
            let t = u / -v;
            let mut hit1 = Hit {
                t, // infinity (this is the C++ comment, but I'm pretty sure this is not infinity)
                entering: true,
                what: self,
                position: ray.position + t * ray.direction,
                normal: Vector::new(self.a, self.b, self.c),
            };
            if hit1.normal.dot(ray.direction) > 0. {
                hit1.normal.negate();
            }
            let hit2 = Hit {
                t: 10000000000., // infinity
                entering: false,
                what: self,
                position: Vertex::default(),
                normal: Vector::default(),
            };
            return vec![hit1, hit2];
        }
    }

    fn apply_transform(&mut self, transform: &Transform) {
        let ti = transform.inverse().transpose();
        let mut v = Vertex::from_xyzw(self.a, self.b, self.c, self.d);

        ti.apply_to(&mut v);

        self.a = v.x;
        self.b = v.y;
        self.c = v.z;
        self.d = v.w;
    }
}
